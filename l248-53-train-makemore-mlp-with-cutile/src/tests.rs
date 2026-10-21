use super::*;

fn download(t: &Tensor<f32>, stream: &Arc<cutile::cuda_core::Stream>) -> Vec<f32> {
    t.dup().to_host_vec().sync_on(stream).unwrap()
}

#[test]
#[ignore = "нужны NVIDIA GPU и CUDA Toolkit с tileiras"]
fn gpu_forward_and_all_gradients_match_cpu() -> Result<()> {
    let device = Device::new(0)?;
    let stream = device.new_stream()?;
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let [train, _, _] = datasets(&mut rng);
    let model = Model::new(&mut rng, &stream)?;
    let c = download(&model.c, &stream);
    let w1 = download(&model.w1, &stream);
    let b1 = download(&model.b1, &stream);
    let w2 = download(&model.w2, &stream);
    let b2 = download(&model.b2, &stream);
    let batch = 64;
    let train = Dataset {
        rows: train.rows[..35 * 4].to_vec(),
    };
    let data = upload(train.rows.clone(), &[train.len(), 4], &stream)?;
    let step = api::zeros::<i32>(&[1]).sync_on(&stream)?;
    let mut b = Buffers::new(batch, &stream)?;
    let mut g = Model::zero(&stream)?;
    let graph = CudaGraph::scope(&stream, |s| {
        forward(s, &model, &mut b, &data, &step, batch, false)?;
        backward(s, &model, &mut b, &mut g, batch)
    })?;
    graph.launch().sync_on(&stream)?;
    let mut dc = vec![0.0f32; c.len()];
    let mut dw1 = vec![0.0f32; w1.len()];
    let mut db1 = vec![0.0f32; b1.len()];
    let mut dw2 = vec![0.0f32; w2.len()];
    let mut db2 = vec![0.0f32; b2.len()];
    let mut losses = Vec::new();
    for row in train.rows.as_chunks::<4>().0.iter().cycle().take(batch) {
        let mut x = [0.0f32; INPUT];
        for slot in 0..3 {
            x[slot * 16..slot * 16 + 16]
                .copy_from_slice(&c[row[slot] as usize * 16..row[slot] as usize * 16 + 16]);
        }
        let mut h = b1.clone();
        for k in 0..INPUT {
            for j in 0..HIDDEN {
                h[j] += x[k] * w1[k * HIDDEN + j];
            }
        }
        for v in &mut h {
            *v = v.tanh();
        }
        let mut z = b2[..VOCAB].to_vec();
        for j in 0..HIDDEN {
            for v in 0..VOCAB {
                z[v] += h[j] * w2[j * 32 + v];
            }
        }
        let max = z.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let sum: f32 = z.iter().map(|z| (z - max).exp()).sum();
        losses.push(max + sum.ln() - z[row[3] as usize]);
        let mut dz = [0.0f32; 32];
        for v in 0..VOCAB {
            dz[v] = ((z[v] - max).exp() / sum - f32::from(v == row[3] as usize)) / batch as f32;
            db2[v] += dz[v];
        }
        let mut da = [0.0f32; HIDDEN];
        for j in 0..HIDDEN {
            for v in 0..VOCAB {
                dw2[j * 32 + v] += h[j] * dz[v];
                da[j] += w2[j * 32 + v] * dz[v];
            }
            da[j] *= 1.0 - h[j] * h[j];
            db1[j] += da[j];
        }
        for k in 0..INPUT {
            let mut dx = 0.0;
            for j in 0..HIDDEN {
                dw1[k * HIDDEN + j] += x[k] * da[j];
                dx += w1[k * HIDDEN + j] * da[j];
            }
            if k < 48 {
                dc[row[k / 16] as usize * 16 + k % 16] += dx;
            }
        }
    }
    let expected_loss = losses[..35].iter().map(|&x| f64::from(x)).sum::<f64>() / 35.0;
    assert!((evaluate(&model, &train, &stream)? - expected_loss).abs() < 2e-5);
    for (name, actual, expected) in [
        ("loss", download(&b.loss, &stream), losses),
        ("dC", download(&g.c, &stream), dc),
        ("dW1", download(&g.w1, &stream), dw1),
        ("db1", download(&g.b1, &stream), db1),
        ("dW2", download(&g.w2, &stream), dw2),
        ("db2", download(&g.b2, &stream), db2),
    ] {
        let mut worst = 0.0f32;
        for (i, (&a, &e)) in actual.iter().zip(&expected).enumerate() {
            worst = worst.max((a - e).abs());
            assert!(
                a.is_finite() && (a - e).abs() <= 2e-4 + 2e-3 * e.abs(),
                "{name}[{i}]: GPU={a}, CPU={e}"
            );
        }
        println!("{name}: max_abs_error={worst:.8}");
    }
    // Повторный replay не должен накапливать старые градиенты.
    let before = download(&g.c, &stream);
    graph.launch().sync_on(&stream)?;
    assert_eq!(before, download(&g.c, &stream));
    // Сэмплирование на GPU должно совпадать с unsigned wrapping hash на CPU.
    let sample_step = upload(vec![1i32], &[1], &stream)?;
    let sampling = CudaGraph::scope(&stream, |s| {
        forward(s, &model, &mut b, &data, &sample_step, batch, true)
    })?;
    sampling.launch().sync_on(&stream)?;
    let ids = b.ids.dup().to_host_vec().sync_on(&stream)?;
    for (i, actual) in ids.as_chunks::<4>().0.iter().enumerate() {
        let mut hash = (batch + i) as u32;
        hash = (hash ^ (hash >> 16)).wrapping_mul(0x7feb352d);
        hash = (hash ^ (hash >> 15)).wrapping_mul(0x846ca68b);
        hash ^= hash >> 16;
        let row = hash as usize % train.len();
        assert_eq!(actual, &train.rows[row * 4..row * 4 + 4]);
    }

    Ok(())
}

#[test]
#[ignore = "нужны NVIDIA GPU и CUDA Toolkit с tileiras"]
fn graph_updates_every_parameter_and_advances_learning_rate() -> Result<()> {
    let device = Device::new(0)?;
    let stream = device.new_stream()?;
    let mut model = Model::zero(&stream)?;
    let gradients = Model {
        c: api::ones(&[32, 16]).sync_on(&stream)?,
        w1: api::ones(&[INPUT, HIDDEN]).sync_on(&stream)?,
        b1: api::ones(&[HIDDEN]).sync_on(&stream)?,
        w2: api::ones(&[HIDDEN, 32]).sync_on(&stream)?,
        b2: api::ones(&[32]).sync_on(&stream)?,
    };
    let mut step = upload(vec![2i32], &[1], &stream)?;
    let graph = CudaGraph::scope(&stream, |s| update(s, &mut model, &gradients, &mut step, 4))?;
    for expected in [-0.3f32, -0.33f32] {
        graph.launch().sync_on(&stream)?;
        for tensor in [&model.c, &model.w1, &model.b1, &model.w2, &model.b2] {
            for actual in download(tensor, &stream) {
                assert!((actual - expected).abs() < 1e-6);
            }
        }
    }
    assert_eq!(step.dup().to_host_vec().sync_on(&stream)?, vec![4]);
    Ok(())
}
