//! GPU-вариант: все шаги SGD исполняются в заранее выделенной памяти.
use cutile::cuda_async::cuda_graph::CudaGraph;
use cutile::prelude::*;
use rand::{RngExt, SeedableRng, seq::SliceRandom};
use rand_distr::{Distribution, StandardNormal};
use std::{collections::BTreeMap, time::Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const INPUT: usize = 64; // 3 × 16 координат и 16 нулей для MMA.
const HIDDEN: usize = 256;
const VOCAB: usize = 27;

struct TrainingConfig {
    batch: usize,
    steps: usize,
}

#[cutile::module]
mod kernels {
    use cutile::core::*;

    #[cutile::entry()]
    fn embedding(
        out: &mut Tensor<f32, { [1, 64] }>,
        ids: &mut Tensor<i32, { [1, 4] }>,
        data: &Tensor<i32, { [-1, 4] }>,
        c: &Tensor<f32, { [32, 16] }>,
        step: &Tensor<i32, { [1] }>,
        batch: i32,
        training: i32,
    ) {
        let pid = get_tile_block_id();
        let s: i32 = tile_to_scalar(step.load_tile(shape![1], [0]).reshape(shape![]));
        let mut row = s * batch + pid.0;
        if training != 0 {
            // Хеш счётчика: выборка с возвращением без CPU-копий на каждом шаге.
            let h_bits: Tile<u32, { [] }> = bitcast(scalar_to_tile::<i32>(row));
            let mut h: u32 = tile_to_scalar(h_bits);
            h = h ^ (h >> 16u32);
            h = h * 2146121005u32;
            h = h ^ (h >> 15u32);
            // 0x846ca68b: JIT 0.4 не разбирает целый литерал > i32::MAX.
            h = h * (1110856517u32 * 2u32 + 1u32);
            h = h ^ (h >> 16u32);
            let n_bits: Tile<u32, { [] }> = bitcast(scalar_to_tile::<i32>(data.shape()[0]));
            let n: u32 = tile_to_scalar(n_bits);
            let row_bits: Tile<i32, { [] }> = bitcast(scalar_to_tile::<u32>(h % n));
            row = tile_to_scalar(row_bits);
        } else {
            row = row % data.shape()[0];
        }
        let labels = data.load_tile(shape![1, 4], [row, 0]);
        ids.store(labels);
        let mut output = out.partition_mut(shape![1, 16]);
        for slot in 0i32..4i32 {
            let zero_embedding: Tile<f32, { [1, 16] }> = constant(0.0f32, shape![1, 16]);
            let mut v: Tile<f32, { [1, 16] }> = zero_embedding;
            if slot < 3 {
                let id: i32 =
                    tile_to_scalar(data.load_tile(shape![1, 1], [row, slot]).reshape(shape![]));
                v = c.load_tile(shape![1, 16], [id, 0]);
            }
            output.store(v, [0, slot]);
        }
    }

    // Универсальный GEMM; транспонирование выполняется внутри тайла.
    // MODE: 1 = linear+tanh; 2 = backward tanh; 0 = обычное умножение.
    #[cutile::entry()]
    fn gemm<const TA: i32, const TB: i32, const MODE: i32>(
        out: &mut Tensor<f32, { [32, 32] }>,
        a: &Tensor<f32, { [-1, -1] }>,
        b: &Tensor<f32, { [-1, -1] }>,
        bias: &Tensor<f32, { [-1] }>,
        activation: &Tensor<f32, { [-1, -1] }>,
        k: i32,
    ) {
        let pid = get_tile_block_id();
        let zero_matrix: Tile<f32, { [32, 32] }> = constant(0.0f32, shape![32, 32]);
        let mut acc = zero_matrix;
        for j in 0i32..(k / 32) {
            let x = if TA != 0 {
                a.load_tile(shape![32, 32], [j, pid.0]).transpose()
            } else {
                a.load_tile(shape![32, 32], [pid.0, j])
            };
            let y = if TB != 0 {
                b.load_tile(shape![32, 32], [pid.1, j]).transpose()
            } else {
                b.load_tile(shape![32, 32], [j, pid.1])
            };
            acc = mma(x, y, acc);
        }
        if MODE == 1 {
            let bs = bias.load_tile(shape![32], [pid.1]).reshape(shape![1, 32]);
            acc = tanh(acc + bs.broadcast(shape![32, 32]));
        }
        if MODE == 2 {
            let h = activation.load_like(out);
            let ones: Tile<f32, { [32, 32] }> = constant(1.0f32, shape![32, 32]);
            acc = acc * (ones - h * h);
        }
        out.store(acc);
    }

    #[cutile::entry()]
    fn softmax(
        grad: &mut Tensor<f32, { [32, 32] }>,
        losses: &mut Tensor<f32, { [32, 1] }>,
        logits: &Tensor<f32, { [-1, 32] }>,
        ids: &Tensor<i32, { [-1, 4] }>,
        bias: &Tensor<f32, { [32] }>,
        batch: f32,
    ) {
        let pid = get_tile_block_id();
        let cols = iota::<i32, { [32] }>(shape![32])
            .reshape(shape![1, 32])
            .broadcast(shape![32, 32]);
        let vocabulary_size: Tile<i32, { [32, 32] }> = constant(27i32, shape![32, 32]);
        let valid = cmpi(cols, vocabulary_size, predicate::LessThan);
        let raw = logits.load_like(grad)
            + bias
                .load_tile(shape![32], [0])
                .reshape(shape![1, 32])
                .broadcast(shape![32, 32]);
        let masked_logit: Tile<f32, { [32, 32] }> = constant(-1.0e30f32, shape![32, 32]);
        let z = select(valid, raw, masked_logit);
        let maximum: Tile<f32, { [32] }> = reduce_max(z, 1);
        let maximum = maximum.reshape(shape![32, 1]);
        let ex = exp(z - maximum.broadcast(shape![32, 32]));
        let sum: Tile<f32, { [32] }> = reduce_sum(ex, 1);
        let sum = sum.reshape(shape![32, 1]);
        let target = ids
            .load_tile(shape![32, 1], [pid.0, 3])
            .broadcast(shape![32, 32]);
        let hot = cmpi(cols, target, predicate::Equal);
        let zeros: Tile<f32, { [32, 32] }> = constant(0.0f32, shape![32, 32]);
        let ones: Tile<f32, { [32, 32] }> = constant(1.0f32, shape![32, 32]);
        let onehot = select(hot, ones, zeros);
        let batch_size: Tile<f32, { [32, 32] }> = batch.broadcast(shape![32, 32]);
        grad.store((ex / sum.broadcast(shape![32, 32]) - onehot) / batch_size);
        let correct: Tile<f32, { [32] }> = reduce_sum(z * onehot, 1);
        let correct = correct.reshape(shape![32, 1]);
        losses.store(maximum + log(sum) - correct);
    }

    #[cutile::entry()]
    fn embedding_grad(
        dc: &mut Tensor<f32, { [1, 16] }>,
        dx: &Tensor<f32, { [-1, 64] }>,
        ids: &Tensor<i32, { [-1, 4] }>,
        batch: i32,
    ) {
        let pid = get_tile_block_id();
        let zero_embedding: Tile<f32, { [16] }> = constant(0.0f32, shape![16]);
        let mut acc = zero_embedding;
        for j in 0i32..(batch / 32) {
            for slot in 0i32..3i32 {
                let id = ids.load_tile(shape![32, 1], [j, slot]);
                let character: Tile<i32, { [32, 1] }> = pid.0.broadcast(shape![32, 1]);
                let mask = cmpi(id, character, predicate::Equal).broadcast(shape![32, 16]);
                let d = dx.load_tile(shape![32, 16], [j, slot]);
                let zeros: Tile<f32, { [32, 16] }> = constant(0.0f32, shape![32, 16]);
                acc = acc + reduce_sum(select(mask, d, zeros), 0);
            }
        }
        dc.store(acc.reshape(shape![1, 16]));
    }

    #[cutile::entry()]
    fn bias_grad(db: &mut Tensor<f32, { [32] }>, grad: &Tensor<f32, { [-1, -1] }>, batch: i32) {
        let zero_bias: Tile<f32, { [32] }> = constant(0.0f32, shape![32]);
        let mut acc = zero_bias;
        for j in 0i32..(batch / 32) {
            acc = acc
                + reduce_sum(
                    grad.load_tile(shape![32, 32], [j, get_tile_block_id().0]),
                    0,
                );
        }
        db.store(acc);
    }

    #[cutile::entry()]
    fn update(
        p: &mut Tensor<f32, { [32] }>,
        g: &Tensor<f32, { [-1] }>,
        step: &Tensor<i32, { [1] }>,
        steps: i32,
    ) {
        let s: i32 = tile_to_scalar(step.load_tile(shape![1], [0]).reshape(shape![]));
        let mut lr = 0.3f32;
        if s >= steps * 3 / 4 {
            lr = 0.03f32;
        }
        let old = p.load();
        let gradient = g.load_like(p);
        let learning_rate: Tile<f32, { [32] }> = lr.broadcast(p.shape());
        p.store(old - gradient * learning_rate);
    }

    #[cutile::entry()]
    fn update_matrix(
        p: &mut Tensor<f32, { [8, 16] }>,
        g: &Tensor<f32, { [-1, -1] }>,
        step: &Tensor<i32, { [1] }>,
        steps: i32,
    ) {
        let s: i32 = tile_to_scalar(step.load_tile(shape![1], [0]).reshape(shape![]));
        let mut lr = 0.3f32;
        if s >= steps * 3 / 4 {
            lr = 0.03f32;
        }
        let old = p.load();
        let gradient = g.load_like(p);
        let learning_rate: Tile<f32, { [8, 16] }> = lr.broadcast(p.shape());
        p.store(old - gradient * learning_rate);
    }

    #[cutile::entry()]
    fn advance(step: &mut Tensor<i32, { [1] }>) {
        let old = step.load();
        let one: Tile<i32, { [1] }> = constant(1i32, shape![1]);
        step.store(old + one);
    }
}

fn upload<T: cutile::DType>(
    data: Vec<T>,
    shape: &[usize],
    stream: &Arc<cutile::cuda_core::Stream>,
) -> Result<Tensor<T>> {
    Ok(api::copy_host_vec_to_device(&Arc::new(data))
        .sync_on(stream)?
        .reshape(shape)?)
}

struct Dataset {
    rows: Vec<i32>,
}
impl Dataset {
    fn len(&self) -> usize {
        self.rows.len() / 4
    }
}

// Тот же split по группам одинаковых имён, что и в CPU-уроке.
fn datasets(rng: &mut rand::rngs::StdRng) -> [Dataset; 3] {
    let mut groups = BTreeMap::<&str, Vec<&str>>::new();
    for name in
        include_str!("../../l247-52-generate-names-with-makemore-mlp/data/names.txt").lines()
    {
        groups.entry(name).or_default().push(name);
    }
    let mut groups: Vec<_> = groups.into_values().collect();
    groups.shuffle(rng);
    let n = groups.len();
    let build = |groups: &[Vec<&str>]| {
        let mut rows = Vec::new();
        for name in groups.iter().flatten() {
            let mut context = [0; 3];
            for c in name.bytes().map(|c| (c - b'a' + 1) as i32).chain([0]) {
                rows.extend(context);
                rows.push(c);
                context.rotate_left(1);
                context[2] = c;
            }
        }
        Dataset { rows }
    };
    [
        build(&groups[..n * 8 / 10]),
        build(&groups[n * 8 / 10..n * 9 / 10]),
        build(&groups[n * 9 / 10..]),
    ]
}

struct Model {
    c: Tensor<f32>,
    w1: Tensor<f32>,
    b1: Tensor<f32>,
    w2: Tensor<f32>,
    b2: Tensor<f32>,
}
impl Model {
    fn new(rng: &mut rand::rngs::StdRng, stream: &Arc<cutile::cuda_core::Stream>) -> Result<Self> {
        let mut normal = |n: usize, scale: f32| -> Vec<f32> {
            (0..n)
                .map(|_| {
                    let x: f32 = StandardNormal.sample(rng);
                    x * scale
                })
                .collect()
        };
        let c = normal(32 * 16, 1.0);
        let mut w1 = normal(INPUT * HIDDEN, (5.0 / 3.0) / (48.0f32).sqrt());
        w1[48 * HIDDEN..].fill(0.0);
        let mut w2 = normal(HIDDEN * 32, 0.01);
        for row in w2.chunks_mut(32) {
            row[VOCAB..].fill(0.0);
        }
        Ok(Self {
            c: upload(c, &[32, 16], stream)?,
            w1: upload(w1, &[INPUT, HIDDEN], stream)?,
            b1: api::zeros(&[HIDDEN]).sync_on(stream)?,
            w2: upload(w2, &[HIDDEN, 32], stream)?,
            b2: api::zeros(&[32]).sync_on(stream)?,
        })
    }
    fn zero(stream: &Arc<cutile::cuda_core::Stream>) -> Result<Self> {
        Ok(Self {
            c: api::zeros(&[32, 16]).sync_on(stream)?,
            w1: api::zeros(&[INPUT, HIDDEN]).sync_on(stream)?,
            b1: api::zeros(&[HIDDEN]).sync_on(stream)?,
            w2: api::zeros(&[HIDDEN, 32]).sync_on(stream)?,
            b2: api::zeros(&[32]).sync_on(stream)?,
        })
    }
}

struct Buffers {
    x: Tensor<f32>,
    h: Tensor<f32>,
    z: Tensor<f32>,
    dz: Tensor<f32>,
    da: Tensor<f32>,
    dx: Tensor<f32>,
    ids: Tensor<i32>,
    loss: Tensor<f32>,
}
impl Buffers {
    fn new(batch: usize, stream: &Arc<cutile::cuda_core::Stream>) -> Result<Self> {
        Ok(Self {
            x: api::zeros(&[batch, INPUT]).sync_on(stream)?,
            h: api::zeros(&[batch, HIDDEN]).sync_on(stream)?,
            z: api::zeros(&[batch, 32]).sync_on(stream)?,
            dz: api::zeros(&[batch, 32]).sync_on(stream)?,
            da: api::zeros(&[batch, HIDDEN]).sync_on(stream)?,
            dx: api::zeros(&[batch, INPUT]).sync_on(stream)?,
            ids: api::zeros(&[batch, 4]).sync_on(stream)?,
            loss: api::zeros(&[batch, 1]).sync_on(stream)?,
        })
    }
}

fn forward(
    scope: &cutile::cuda_async::cuda_graph::Scope,
    model: &Model,
    b: &mut Buffers,
    data: &Tensor<i32>,
    step: &Tensor<i32>,
    batch: usize,
    training: bool,
) -> std::result::Result<(), DeviceError> {
    scope.record(kernels::embedding(
        (&mut b.x).partition([1, 64]),
        (&mut b.ids).partition([1, 4]),
        data,
        &model.c,
        step,
        batch as i32,
        i32::from(training),
    ))?;
    scope.record(
        kernels::gemm(
            (&mut b.h).partition([32, 32]),
            &b.x,
            &model.w1,
            &model.b1,
            &b.x,
            INPUT as i32,
        )
        .generics(vec!["0".into(), "0".into(), "1".into()]),
    )?;
    scope.record(
        kernels::gemm(
            (&mut b.z).partition([32, 32]),
            &b.h,
            &model.w2,
            &model.b2,
            &b.x,
            HIDDEN as i32,
        )
        .generics(vec!["0".into(), "0".into(), "0".into()]),
    )?;
    scope.record(kernels::softmax(
        (&mut b.dz).partition([32, 32]),
        (&mut b.loss).partition([32, 1]),
        &b.z,
        &b.ids,
        &model.b2,
        batch as f32,
    ))?;
    Ok(())
}

fn backward(
    s: &cutile::cuda_async::cuda_graph::Scope,
    model: &Model,
    buffers: &mut Buffers,
    gradients: &mut Model,
    batch: usize,
) -> std::result::Result<(), DeviceError> {
    s.record(
        kernels::gemm(
            (&mut buffers.da).partition([32, 32]),
            &buffers.dz,
            &model.w2,
            &model.b1,
            &buffers.h,
            32,
        )
        .generics(vec!["0".into(), "1".into(), "2".into()]),
    )?;
    s.record(
        kernels::gemm(
            (&mut buffers.dx).partition([32, 32]),
            &buffers.da,
            &model.w1,
            &model.b1,
            &buffers.x,
            HIDDEN as i32,
        )
        .generics(vec!["0".into(), "1".into(), "0".into()]),
    )?;
    s.record(
        kernels::gemm(
            (&mut gradients.w2).partition([32, 32]),
            &buffers.h,
            &buffers.dz,
            &model.b2,
            &buffers.x,
            batch as i32,
        )
        .generics(vec!["1".into(), "0".into(), "0".into()]),
    )?;
    s.record(
        kernels::gemm(
            (&mut gradients.w1).partition([32, 32]),
            &buffers.x,
            &buffers.da,
            &model.b1,
            &buffers.x,
            batch as i32,
        )
        .generics(vec!["1".into(), "0".into(), "0".into()]),
    )?;
    s.record(kernels::embedding_grad(
        (&mut gradients.c).partition([1, 16]),
        &buffers.dx,
        &buffers.ids,
        batch as i32,
    ))?;
    s.record(kernels::bias_grad(
        (&mut gradients.b1).partition([32]),
        &buffers.da,
        batch as i32,
    ))?;
    s.record(kernels::bias_grad(
        (&mut gradients.b2).partition([32]),
        &buffers.dz,
        batch as i32,
    ))?;
    Ok(())
}

fn update(
    s: &cutile::cuda_async::cuda_graph::Scope,
    model: &mut Model,
    gradients: &Model,
    step: &mut Tensor<i32>,
    steps: usize,
) -> std::result::Result<(), DeviceError> {
    for (p, g) in [
        (&mut model.c, &gradients.c),
        (&mut model.w1, &gradients.w1),
        (&mut model.w2, &gradients.w2),
    ] {
        s.record(kernels::update_matrix(
            p.partition([8, 16]),
            g,
            &*step,
            steps as i32,
        ))?;
    }
    for (p, g) in [
        (&mut model.b1, &gradients.b1),
        (&mut model.b2, &gradients.b2),
    ] {
        s.record(kernels::update(p.partition([32]), g, &*step, steps as i32))?;
    }
    s.record(kernels::advance((&mut *step).partition([1])))?;
    Ok(())
}

fn training_graph(
    model: &mut Model,
    buffers: &mut Buffers,
    gradients: &mut Model,
    data: &Tensor<i32>,
    step: &mut Tensor<i32>,
    config: &TrainingConfig,
    stream: &Arc<cutile::cuda_core::Stream>,
) -> Result<CudaGraph<()>> {
    let TrainingConfig { batch, steps } = *config;
    Ok(CudaGraph::scope(stream, |s| {
        // Несколько последовательных шагов в одном replay уменьшают затраты CPU.
        for _ in 0..10 {
            forward(s, model, buffers, data, step, batch, true)?;
            backward(s, model, buffers, gradients, batch)?;
            update(s, model, gradients, step, steps)?;
        }
        Ok(())
    })?)
}

fn evaluate(
    model: &Model,
    dataset: &Dataset,
    stream: &Arc<cutile::cuda_core::Stream>,
) -> Result<f64> {
    let batch = 1024;
    let data = upload(dataset.rows.clone(), &[dataset.len(), 4], stream)?;
    let mut b = Buffers::new(batch, stream)?;
    let mut step = api::zeros::<i32>(&[1]).sync_on(stream)?;
    let graph = CudaGraph::scope(stream, |s| {
        forward(s, model, &mut b, &data, &step, batch, false)?;
        s.record(kernels::advance((&mut step).partition([1])))?;
        Ok(())
    })?;
    let mut total = 0.0f64;
    for offset in (0..dataset.len()).step_by(batch) {
        graph.launch().sync_on(stream)?;
        let loss = b.loss.dup().to_host_vec().sync_on(stream)?;
        total += loss[..batch.min(dataset.len() - offset)]
            .iter()
            .map(|&x| f64::from(x))
            .sum::<f64>();
    }
    let mean = total / dataset.len() as f64;
    if !mean.is_finite() {
        return Err("нечисловой loss при оценке".into());
    }
    Ok(mean)
}

fn main() -> Result<()> {
    let started = Instant::now();
    let mut steps = 10_000usize;
    let mut batch = 128usize;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--steps" => steps = args.next().ok_or("нет значения --steps")?.parse()?,
            "--batch" => batch = args.next().ok_or("нет значения --batch")?.parse()?,
            _ => return Err(format!("неизвестный GPU-аргумент: {arg}").into()),
        }
    }
    if steps == 0
        || !steps.is_multiple_of(10)
        || batch < 32
        || !batch.is_multiple_of(32)
        || batch > 4096
        || steps
            .checked_mul(batch)
            .is_none_or(|n| n > i32::MAX as usize)
    {
        return Err("steps должен быть положительным и кратным 10; batch — кратным 32 в 32..=4096; steps*batch <= i32::MAX".into());
    }
    let device = Device::new(0)?;
    let stream = device.new_stream()?;
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let [train, dev, test] = datasets(&mut rng);
    println!(
        "cuTile: batch={batch}, steps={steps}, embedding=16, hidden={HIDDEN}; train/dev/test={}/{}/{}",
        train.len(),
        dev.len(),
        test.len()
    );
    let mut model = Model::new(&mut rng, &stream)?;
    let initial = evaluate(&model, &dev, &stream)?;
    println!("Начальный dev loss={initial:.6}");
    let data = upload(train.rows.clone(), &[train.len(), 4], &stream)?;
    let mut buffers = Buffers::new(batch, &stream)?;
    let mut gradients = Model::zero(&stream)?;
    let mut step = api::zeros::<i32>(&[1]).sync_on(&stream)?;
    let graph = training_graph(
        &mut model,
        &mut buffers,
        &mut gradients,
        &data,
        &mut step,
        &TrainingConfig { batch, steps },
        &stream,
    )?;
    println!("JIT и подготовка: {:.3}s", started.elapsed().as_secs_f64());
    let training = Instant::now();
    for i in 0..steps / 10 {
        graph.launch().sync_on(&stream)?;
        if (i + 1) % 100 == 0 || i + 1 == steps / 10 {
            let losses = buffers.loss.dup().to_host_vec().sync_on(&stream)?;
            let loss = losses.iter().sum::<f32>() / batch as f32;
            if !loss.is_finite() {
                return Err("нечисловой loss".into());
            }
            println!(
                "step={} loss={loss:.6} elapsed={:.3}s",
                (i + 1) * 10,
                training.elapsed().as_secs_f64()
            );
        }
    }
    let elapsed = training.elapsed().as_secs_f64();
    println!(
        "Обучение: {elapsed:.3}s; {:.1} steps/s",
        steps as f64 / elapsed
    );
    for (name, data) in [("train", &train), ("dev", &dev), ("test", &test)] {
        println!("{name}_loss={:.6}", evaluate(&model, data, &stream)?);
    }
    generate(&model, &mut rng, &stream)?;
    println!("Общее время: {:.3}s", started.elapsed().as_secs_f64());
    Ok(())
}

// Для последовательной генерации 20 имён копируем веса один раз: маленький
// одиночный forward на CPU дешевле запуска GPU на каждый символ.
fn generate(
    model: &Model,
    rng: &mut rand::rngs::StdRng,
    stream: &Arc<cutile::cuda_core::Stream>,
) -> Result<()> {
    let c = model.c.dup().to_host_vec().sync_on(stream)?;
    let w1 = model.w1.dup().to_host_vec().sync_on(stream)?;
    let b1 = model.b1.dup().to_host_vec().sync_on(stream)?;
    let w2 = model.w2.dup().to_host_vec().sync_on(stream)?;
    let b2 = model.b2.dup().to_host_vec().sync_on(stream)?;
    for _ in 0..20 {
        let mut context = [0usize; 3];
        let mut name = String::new();
        for position in 0..100 {
            let mut h = b1.clone();
            for slot in 0..3 {
                for d in 0..16 {
                    let x = c[context[slot] * 16 + d];
                    for j in 0..HIDDEN {
                        h[j] += x * w1[(slot * 16 + d) * HIDDEN + j];
                    }
                }
            }
            for x in &mut h {
                *x = x.tanh();
            }
            let mut z = b2[..VOCAB].to_vec();
            for j in 0..HIDDEN {
                for k in 0..VOCAB {
                    z[k] += h[j] * w2[j * 32 + k];
                }
            }
            let max = z.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            for x in &mut z {
                *x = (*x - max).exp();
            }
            let mut sample = rng.random::<f32>() * z.iter().sum::<f32>();
            let mut next = VOCAB - 1;
            for (i, p) in z.iter().enumerate() {
                sample -= p;
                if sample <= 0.0 {
                    next = i;
                    break;
                }
            }
            if next == 0 {
                break;
            }
            name.push((b'a' + next as u8 - 1) as char);
            context.rotate_left(1);
            context[2] = next;
            if position == 99 {
                name.push_str(" [обрезано]");
            }
        }
        println!("{name}");
    }
    Ok(())
}

#[cfg(test)]
mod tests;
