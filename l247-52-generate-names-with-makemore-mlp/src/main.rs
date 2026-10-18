mod model;
use model::{Model, Rng, cross_entropy, dataset};
use std::{error::Error, fs, io::Write, path::PathBuf};

// SVG строится самим Rust; ни matplotlib, ни дополнительные программы не нужны.
fn plot(
    path: PathBuf,
    points: &[(f32, f32)],
    labels: Option<&[char]>,
    title: &str,
) -> std::io::Result<()> {
    let minx = points.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
    let maxx = points.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max);
    let miny = points.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
    let maxy = points.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);
    let xy = |p: &(f32, f32)| {
        (
            65.0 + 680.0 * (p.0 - minx) / (maxx - minx).max(1e-6),
            535.0 - 470.0 * (p.1 - miny) / (maxy - miny).max(1e-6),
        )
    };
    let mut s = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='800' height='600' viewBox='0 0 800 600'><rect width='800' height='600' fill='white'/><g font-family='sans-serif' fill='#172554'><text x='65' y='28'>{title}</text><path d='M65 55V535H755' fill='none' stroke='#64748b'/><text x='65' y='565'>{minx:.3}</text><text x='690' y='565'>{maxx:.3}</text><text x='5' y='535'>{miny:.3}</text><text x='5' y='65'>{maxy:.3}</text>"
    );
    if let Some(labels) = labels {
        for (p, ch) in points.iter().zip(labels) {
            let (x, y) = xy(p);
            s.push_str(&format!("<circle cx='{x}' cy='{y}' r='12' fill='#2563eb'/><text x='{x}' y='{}' text-anchor='middle' fill='white'>{ch}</text>", y+4.0));
        }
    } else {
        s.push_str("<polyline fill='none' stroke='#2563eb' stroke-width='0.7' points='");
        for p in points {
            let (x, y) = xy(p);
            s.push_str(&format!("{x:.2},{y:.2} "));
        }
        s.push_str("'/>");
    }
    s.push_str("</g></svg>");
    fs::write(path, s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut steps = 200_000usize;
    let mut sweep = false;
    let mut out = root.join("output");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--steps" => steps = args.next().ok_or("после --steps нужно число")?.parse()?,
            "--out" => out = args.next().ok_or("после --out нужен путь")?.into(),
            "--lr-sweep" => sweep = true,
            "--help" => {
                println!(
                    "cargo run --release -p l247-52-generate-names-with-makemore-mlp -- [--steps 200000] [--lr-sweep] [--out PATH]"
                );
                return Ok(());
            }
            _ => return Err(format!("неизвестный аргумент: {arg}").into()),
        }
    }
    if steps == 0 {
        return Err("--steps должен быть положительным".into());
    }
    fs::create_dir_all(&out)?;
    // 1. Данные и словарь: 0 — начало/конец имени, a..z — 1..26.
    let text = include_str!("../data/names.txt");
    let mut words: Vec<_> = text.lines().collect();
    let mut chars: Vec<_> = text.chars().filter(|c| c.is_ascii_lowercase()).collect();
    chars.sort_unstable();
    chars.dedup();
    chars.insert(0, '.');
    assert!(
        words
            .iter()
            .all(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_lowercase()))
    );
    println!("Имена: {}, первые восемь: {:?}", words.len(), &words[..8]);
    println!("Словарь: {chars:?}");
    let all = dataset(&words, &chars);
    println!("Все X: ({},3), Y: ({},)", all.len(), all.len());
    for e in dataset(&words[..1], &chars) {
        println!(
            "{} -> {}",
            e.context.map(|i| chars[i]).iter().collect::<String>(),
            chars[e.target]
        );
    }
    // 2. Маленькая сеть из начала notebook: C(27,2), W1(6,100).
    let demo = Model::new(chars.len(), 2, 100, &mut Rng::new(2147483647));
    let (x, h, z) = demo.forward(all[0].context);
    let (stable, p) = cross_entropy(&z, all[0].target);
    let counts: Vec<_> = z.iter().map(|v| v.exp()).collect();
    let naive = -(counts[all[0].target] / counts.iter().sum::<f32>()).ln();
    println!(
        "Демо: emb=(1,3,2), flat=(1,{}), h=(1,{}), logits=(1,{}); sum(prob)={:.6}",
        x.len(),
        h.len(),
        z.len(),
        p.iter().sum::<f32>()
    );
    println!(
        "Один пример: явный softmax+NLL={naive:.6}, stable CE={stable:.6}; loss первых 32={:.6}",
        demo.loss(&all[..32])
    );
    // 3. Разделяем ИМЕНА до построения контекстов: 80/10/10.
    Rng::new(42).shuffle(&mut words);
    let n1 = words.len() * 8 / 10;
    let n2 = words.len() * 9 / 10;
    let train = dataset(&words[..n1], &chars);
    let dev = dataset(&words[n1..n2], &chars);
    let test = dataset(&words[n2..], &chars);
    println!(
        "Имена train/dev/test: {}/{}/{}; примеры: {}/{}/{}",
        n1,
        n2 - n1,
        words.len() - n2,
        train.len(),
        dev.len(),
        test.len()
    );
    let mut model = Model::new(chars.len(), 10, 200, &mut Rng::new(2147483647));
    println!(
        "C=({},{}), W1=({},{}), b1=({}), W2=({},{}), b2=({}); параметров: {}",
        model.v,
        model.d,
        3 * model.d,
        model.h,
        model.h,
        model.h,
        model.v,
        model.v,
        model.p.iter().map(Vec::len).sum::<usize>()
    );
    // 4. Необязательный опыт с lr: отдельная копия, основной запуск не меняется.
    if sweep {
        let mut trial = model.clone();
        let mut rng = Rng::new(123);
        let mut points = Vec::new();
        let mut csv = fs::File::create(out.join("lr_sweep.csv"))?;
        writeln!(csv, "log10_lr,lr,loss")?;
        for i in 0..1000 {
            let exponent = -3.0 + 3.0 * i as f32 / 999.0;
            let lr = 10.0f32.powf(exponent);
            let batch: Vec<_> = (0..32).map(|_| train[rng.index(train.len())]).collect();
            let loss = trial.step(&batch, lr);
            writeln!(csv, "{exponent},{lr},{loss}")?;
            points.push((exponent, loss));
        }
        plot(
            out.join("lr_sweep.svg"),
            &points,
            None,
            "Learning-rate sweep: x=log10(lr), y=loss",
        )?;
    }
    // 5. SGD: выборка с возвращением, средняя CE, очистка градиентов каждый шаг.
    let initial = model.loss(&dev);
    println!("До обучения dev loss={initial:.6}");
    let mut rng = Rng::new(2147483647);
    let mut csv = std::io::BufWriter::new(fs::File::create(out.join("loss.csv"))?);
    writeln!(csv, "step,learning_rate,loss,log10_loss")?;
    let mut curve = Vec::with_capacity(steps);
    let started = std::time::Instant::now();
    for i in 0..steps {
        let batch: Vec<_> = (0..32).map(|_| train[rng.index(train.len())]).collect();
        let lr = if i < 100_000 { 0.1 } else { 0.01 };
        let loss = model.step(&batch, lr);
        if !loss.is_finite() {
            return Err(format!("loss не конечен на шаге {i}").into());
        }
        writeln!(csv, "{i},{lr},{loss},{}", loss.log10())?;
        curve.push((i as f32, loss.log10()));
        if i % 10_000 == 0 || i + 1 == steps {
            println!(
                "step={i:6} lr={lr} loss={loss:.4} elapsed={:.1}s",
                started.elapsed().as_secs_f32()
            );
        }
    }
    csv.flush()?;
    // 6. Оценка без обновлений и графики, как в notebook.
    let train_loss = model.loss(&train);
    let dev_loss = model.loss(&dev);
    // Test — только окончательная оценка, не выбор lr/архитектуры.
    let test_loss = model.loss(&test);
    let parameters = model.p.iter().map(Vec::len).sum::<usize>();
    let metrics = format!(
        "steps={steps}\nparameters={parameters}\ninitial_dev_loss={initial:.6}\ntrain_loss={train_loss:.6}\ndev_loss={dev_loss:.6}\ntest_loss={test_loss:.6}\n"
    );
    print!("{metrics}");
    fs::write(out.join("metrics.txt"), metrics)?;
    plot(
        out.join("loss.svg"),
        &curve,
        None,
        "Training: x=step, y=log10(minibatch loss)",
    )?;
    let embedding: Vec<_> = (0..chars.len())
        .map(|i| (model.p[0][i * model.d], model.p[0][i * model.d + 1]))
        .collect();
    plot(
        out.join("embeddings.svg"),
        &embedding,
        Some(&chars),
        "C: x=dimension 0, y=dimension 1 (of 10)",
    )?;
    let mut ecsv = fs::File::create(out.join("embeddings.csv"))?;
    writeln!(ecsv, "character,c0,c1")?;
    for (ch, (x, y)) in chars.iter().zip(embedding) {
        writeln!(ecsv, "{ch},{x},{y}")?;
    }
    // 7. Авторегрессионная генерация с собственным фиксированным seed.
    let mut sampling = Rng::new(2147483647 + 10);
    let mut names = String::new();
    for _ in 0..20 {
        let name = model.sample(&chars, &mut sampling);
        println!("{name}");
        names.push_str(&name);
        names.push('\n');
    }
    fs::write(out.join("samples.txt"), names)?;
    println!("Артефакты: {}", out.display());
    Ok(())
}
