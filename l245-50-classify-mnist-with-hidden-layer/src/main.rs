fn main() -> Result<(), String> {
    // 1. PNG -> normalized pixels. Folder names are the labels.
    let load_digits = |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
        let mut digits = Vec::new();
        for label in 0..10u8 {
            let class = directory.join(label.to_string());
            let entries = std::fs::read_dir(&class).map_err(|e| {
                format!(
                    "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                    class.display()
                )
            })?;
            let mut paths = Vec::new();
            for entry in entries {
                let path = entry.map_err(|e| e.to_string())?.path();
                if path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
                {
                    paths.push(path);
                }
            }
            paths.sort();
            if paths.is_empty() {
                return Err(format!("{}: нет PNG", class.display()));
            }
            for path in paths {
                let decoded = (|| -> Result<[f64; 784], String> {
                    let file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
                    let mut reader = png::Decoder::new(std::io::BufReader::new(file))
                        .read_info()
                        .map_err(|e| e.to_string())?;
                    let info = reader.info();
                    if info.width != 28
                        || info.height != 28
                        || info.color_type != png::ColorType::Grayscale
                        || info.bit_depth != png::BitDepth::Eight
                        || info.animation_control.is_some()
                    {
                        return Err("Ожидается статический PNG 28×28, grayscale, 8 бит".into());
                    }
                    let mut pixels = [0u8; 784];
                    reader.next_frame(&mut pixels).map_err(|e| e.to_string())?;
                    reader.finish().map_err(|e| e.to_string())?;
                    Ok(std::array::from_fn(|i| f64::from(pixels[i]) / 255.0))
                })()
                .map_err(|e| format!("{}: {e}", path.display()))?;
                digits.push((decoded, label));
            }
        }
        Ok(digits)
    };
    use ndarray::{Array1, Array2, Axis};
    // Parameters: one (weights, bias) pair for each trainable layer.
    type Layer = (Array2<f32>, Array1<f32>);
    let next_random = |state: &mut u64| {
        *state = state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    };
    let shuffle = |items: &mut [usize], state: &mut u64| {
        for i in (1..items.len()).rev() {
            let j = (next_random(state) % (i as u64 + 1)) as usize;
            items.swap(i, j);
        }
    };
    let new_layer = |input: usize, output: usize, state: &mut u64| -> Layer {
        let bound = (6.0 / input as f32).sqrt();
        (
            Array2::from_shape_fn((input, output), |_| {
                let uniform = (next_random(state) >> 40) as f32 / 16777216.0;
                (2.0 * uniform - 1.0) * bound
            }),
            Array1::zeros(output),
        )
    };
    let dense_backward = |input: &Array2<f32>, weights: &Array2<f32>, gradient: &Array2<f32>| {
        (
            gradient.dot(&weights.t()),
            input.t().dot(gradient),
            gradient.sum_axis(Axis(0)),
        )
    };
    let cross_entropy = |scores: &Array2<f32>, labels: &[u8]| -> (f32, Array2<f32>) {
        assert_eq!(scores.nrows(), labels.len());
        assert!(!labels.is_empty());
        let mut gradient = scores.clone();
        let mut loss = 0.0;
        for (mut row, &label) in gradient.rows_mut().into_iter().zip(labels) {
            assert!((label as usize) < row.len());
            let max = row.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let target = row[label as usize];
            row.mapv_inplace(|x| (x - max).exp());
            let sum = row.sum();
            loss += max + sum.ln() - target;
            row /= sum;
            row[label as usize] -= 1.0;
            row /= labels.len() as f32;
        }
        (loss / labels.len() as f32, gradient)
    };
    let argmax = |row: ndarray::ArrayView1<'_, f32>| -> usize {
        (0..row.len())
            .max_by(|&a, &b| row[a].total_cmp(&row[b]).then_with(|| b.cmp(&a)))
            .unwrap()
    };
    let relu = |values: &Array2<f32>| values.mapv(|x| x.max(0.0));
    let relu_backward = |activated: &Array2<f32>, gradient: &Array2<f32>| {
        assert_eq!(activated.dim(), gradient.dim());
        ndarray::Zip::from(activated)
            .and(gradient)
            .map_collect(|&x, &g| if x > 0.0 { g } else { 0.0 })
    };
    // 2. Experiment settings; the training loop below is directly in main.
    let start = std::time::Instant::now();
    let mut epochs = 15usize;
    let mut batch_size = 64usize;
    let mut learning_rate = 0.001f32;
    let mut seed = 42u64;
    let mut train_limit: Option<usize> = None;
    let mut data =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
    let mut self_check = false;
    let mut exercise = false;
    let mut arguments = std::env::args().skip(1);
    while let Some(key) = arguments.next() {
        if key == "--exercise" {
            exercise = true;
            continue;
        }
        if key == "--self-check" {
            self_check = true;
            continue;
        }
        if key == "--help" {
            println!(
                "--epochs N --batch-size N --learning-rate X --seed N --train-limit N --data PNG_ROOT --self-check --exercise"
            );
            return Ok(());
        }
        let value = arguments
            .next()
            .ok_or_else(|| format!("Нет значения для {key}"))?;
        let invalid = || format!("Неверное значение {key}: {value}");
        match key.as_str() {
            "--epochs" => epochs = value.parse().map_err(|_| invalid())?,
            "--batch-size" => batch_size = value.parse().map_err(|_| invalid())?,
            "--learning-rate" => learning_rate = value.parse().map_err(|_| invalid())?,
            "--seed" => seed = value.parse().map_err(|_| invalid())?,
            "--train-limit" => train_limit = Some(value.parse().map_err(|_| invalid())?),
            "--data" => data = value.into(),
            _ => return Err(format!("Неизвестный аргумент {key}")),
        }
    }
    if epochs == 0
        || batch_size == 0
        || train_limit == Some(0)
        || !learning_rate.is_finite()
        || learning_rate <= 0.0
    {
        return Err("Настройки обучения должны быть положительными и конечными".into());
    }
    // Forward pass: values needed for backward are retained locally.
    let forward = |layers: &[Layer],
                   input: &Array2<f32>|
     -> (Vec<Array2<f32>>, Vec<Array2<f32>>, Vec<Array2<usize>>) {
        let hidden = relu(&(input.dot(&layers[0].0) + &layers[0].1));
        let scores = hidden.dot(&layers[1].0) + &layers[1].1;
        (vec![input.clone(), hidden, scores], Vec::new(), Vec::new())
    };
    let batch = |digits: &[([f64; 784], u8)], examples: &[usize]| {
        (
            Array2::from_shape_fn((examples.len(), 784), |(n, p)| {
                digits[examples[n]].0[p] as f32
            }),
            examples.iter().map(|&i| digits[i].1).collect::<Vec<_>>(),
        )
    };
    let evaluate =
        |layers: &[Layer], digits: &[([f64; 784], u8)], examples: &[usize], batch_size: usize| {
            assert!(!examples.is_empty() && batch_size > 0);
            let mut confusion = [[0usize; 10]; 10];
            let mut loss = 0.0;
            for indices in examples.chunks(batch_size) {
                let (input, labels) = batch(digits, indices);
                let (states, _, _) = forward(layers, &input);
                let scores = states.last().unwrap();
                loss += cross_entropy(scores, &labels).0 * indices.len() as f32;
                for (row, label) in scores.rows().into_iter().zip(labels) {
                    confusion[label as usize][argmax(row)] += 1;
                }
            }
            (
                loss / examples.len() as f32,
                (0..10).map(|i| confusion[i][i]).sum::<usize>() as f32 / examples.len() as f32,
                confusion,
            )
        };
    if exercise {
        // Самостоятельное задание: Чему равна производная ReLU при входе -2?
        let answer: Option<f32> = None;
        assert_eq!(
            answer.expect("реши вопрос и замени None на Some(ответ)"),
            0.0
        );
        return Ok(());
    }

    if self_check {
        // Exercise the real PNG loader with temporary images, including bad inputs.
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let temporary =
            std::env::temp_dir().join(format!("mnist-main-{}-{unique}", std::process::id()));
        let checked = (|| -> Result<(), String> {
            assert!(load_digits(&temporary).is_err());
            for label in 0..10u8 {
                let class = temporary.join(label.to_string());
                std::fs::create_dir_all(&class).map_err(|e| e.to_string())?;
                for (name, value) in [("00002.png", 255u8), ("00001.png", 0u8)] {
                    let file =
                        std::fs::File::create(class.join(name)).map_err(|e| e.to_string())?;
                    let mut encoder = png::Encoder::new(file, 28, 28);
                    encoder.set_color(png::ColorType::Grayscale);
                    encoder.set_depth(png::BitDepth::Eight);
                    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
                    writer
                        .write_image_data(&[value; 784])
                        .map_err(|e| e.to_string())?;
                    writer.finish().map_err(|e| e.to_string())?;
                }
                std::fs::write(class.join("notes.txt"), "ignored").map_err(|e| e.to_string())?;
            }
            let decoded = load_digits(&temporary)?;
            assert_eq!(decoded.len(), 20);
            for (i, (pixels, label)) in decoded.iter().enumerate() {
                assert_eq!(*label as usize, i / 2);
                assert_eq!(*pixels, [(i % 2) as f64; 784]);
            }
            let path = temporary.join("0/00001.png");
            std::fs::write(&path, b"broken PNG").map_err(|e| e.to_string())?;
            assert!(load_digits(&temporary).err().unwrap().contains("00001.png"));
            for (width, color, channels) in [
                (27, png::ColorType::Grayscale, 1),
                (28, png::ColorType::Rgb, 3),
            ] {
                let file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
                let mut encoder = png::Encoder::new(file, width, 28);
                encoder.set_color(color);
                encoder.set_depth(png::BitDepth::Eight);
                let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
                writer
                    .write_image_data(&vec![0; width as usize * 28 * channels])
                    .map_err(|e| e.to_string())?;
                writer.finish().map_err(|e| e.to_string())?;
                assert!(load_digits(&temporary).is_err());
            }
            Ok(())
        })();
        if temporary.exists() {
            std::fs::remove_dir_all(&temporary).map_err(|e| e.to_string())?;
        }
        checked?;
    }
    if self_check {
        // Numerical derivative checks for the actual local operations.
        let close = |actual: f32, expected: f32| {
            assert!(
                (actual - expected).abs() < 0.004 * (1.0 + expected.abs()),
                "{actual} != {expected}"
            )
        };
        let h = 0.002f32;
        let mut scores = ndarray::array![[0.2, -0.3, 0.8], [-0.2, 0.7, 0.1]];
        let (_, gradient) = cross_entropy(&scores, &[2, 1]);
        for n in 0..2 {
            for p in 0..3 {
                let original = scores[[n, p]];
                scores[[n, p]] = original + h;
                let plus = cross_entropy(&scores, &[2, 1]).0;
                scores[[n, p]] = original - h;
                let minus = cross_entropy(&scores, &[2, 1]).0;
                scores[[n, p]] = original;
                close(gradient[[n, p]], (plus - minus) / (2.0 * h));
            }
        }
        assert!(
            cross_entropy(&ndarray::array![[10000.0, 9999.0, -10000.0]], &[0])
                .0
                .is_finite()
        );
        let input = ndarray::array![[0.2, -0.4, 0.8], [0.7, 0.3, -0.1]];
        let incoming = ndarray::array![[0.2, -0.3], [0.4, 0.5]];
        let mut state = 42;
        let mut layer = new_layer(3, 2, &mut state);
        let (_, dw, _) = dense_backward(&input, &layer.0, &incoming);
        for i in 0..3 {
            for j in 0..2 {
                let original = layer.0[[i, j]];
                layer.0[[i, j]] = original + h;
                let plus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                layer.0[[i, j]] = original - h;
                let minus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                layer.0[[i, j]] = original;
                close(dw[[i, j]], (plus - minus) / (2.0 * h));
            }
        }
        assert_eq!(
            relu_backward(
                &relu(&ndarray::array![[-1.0, 0.0, 2.0]]),
                &ndarray::array![[3.0, 4.0, 5.0]]
            ),
            ndarray::array![[0.0, 0.0, 5.0]]
        );

        let mut input = ndarray::array![[0.2, -0.4, 0.8], [0.7, 0.3, -0.1]];
        let incoming = ndarray::array![[0.2, -0.3], [0.4, 0.5]];
        let mut state = 42;
        let mut layer = new_layer(3, 2, &mut state);
        let (dx, _, db) = dense_backward(&input, &layer.0, &incoming);
        for j in 0..2 {
            layer.1[j] += h;
            let plus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
            layer.1[j] -= 2.0 * h;
            let minus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
            layer.1[j] += h;
            close(db[j], (plus - minus) / (2.0 * h));
        }
        for n in 0..2 {
            for p in 0..3 {
                let original = input[[n, p]];
                input[[n, p]] = original + h;
                let plus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                input[[n, p]] = original - h;
                let minus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                input[[n, p]] = original;
                close(dx[[n, p]], (plus - minus) / (2.0 * h));
            }
        }
        let mut a = 42;
        let mut b = 42;
        let mut c = 43;
        assert_eq!(new_layer(10, 3, &mut a), new_layer(10, 3, &mut b));
        assert_ne!(new_layer(10, 3, &mut a), new_layer(10, 3, &mut c));
        let mut first: Vec<_> = (0..20).collect();
        let mut second = first.clone();
        a = 42;
        b = 42;
        shuffle(&mut first, &mut a);
        shuffle(&mut second, &mut b);
        assert_eq!(first, second);
        first.sort();
        assert_eq!(first, (0..20).collect::<Vec<_>>());
        epochs = 20;
        batch_size = 40;
        learning_rate = 0.001;
        seed = 42;
        train_limit = None;
    }
    println!(
        "MLP 784 -> 128 -> ReLU -> 10: epochs={epochs}, batch={batch_size}, lr={learning_rate}, seed={seed}, data={}",
        data.display()
    );
    let digits: Vec<([f64; 784], u8)> = if self_check {
        // Fifty synthetic images: the same pipeline, no downloads required.
        (0..10u8)
            .flat_map(|label| {
                (0..5).map(move |_| {
                    let pixels = std::array::from_fn(|p| {
                        let y = p / 28;
                        let x = p % 28;
                        let n = label as usize;
                        if x >= 2 + (n % 5) * 5
                            && x < 5 + (n % 5) * 5
                            && y >= 3 + (n / 5) * 12
                            && y < 10 + (n / 5) * 12
                        {
                            1.0
                        } else {
                            0.0
                        }
                    });
                    (pixels, label)
                })
            })
            .collect()
    } else {
        load_digits(&data.join("train"))?
    };
    // 3. Same split for all lessons, before training or shuffling.
    let mut class_counts = [0usize; 10];
    let (mut training, mut validation) = (Vec::new(), Vec::new());
    for (index, (_, label)) in digits.iter().enumerate() {
        let count = &mut class_counts[*label as usize];
        if *count % 5 == 0 {
            validation.push(index);
        } else {
            training.push(index);
        }
        *count += 1;
    }
    if self_check {
        assert!(training.iter().all(|i| !validation.contains(i)));
    }
    let mut random_state = seed;
    shuffle(&mut training, &mut random_state);
    if let Some(limit) = train_limit {
        training.truncate(limit);
    }
    let mut counts = [0usize; 10];
    for &index in &training {
        counts[digits[index].1 as usize] += 1;
    }
    if counts.contains(&0) {
        return Err("Train должен содержать все классы; увеличь --train-limit".into());
    }
    let majority = (0..10)
        .max_by_key(|&i| (counts[i], std::cmp::Reverse(i)))
        .unwrap();
    println!(
        "train={}, validation={}, baseline digit={majority}; split=every fifth per class, sorted PNG filenames",
        training.len(),
        validation.len()
    );
    let mut layers = vec![
        new_layer(784, 128, &mut random_state),
        new_layer(128, 10, &mut random_state),
    ];
    let initial_layers = layers.clone();
    let zero_moments = || {
        layers
            .iter()
            .map(|(w, b)| (Array2::zeros(w.dim()), Array1::zeros(b.dim())))
            .collect::<Vec<Layer>>()
    };
    let mut first_moments = zero_moments();
    let mut second_moments = zero_moments();
    let initial = evaluate(&layers, &digits, &validation, batch_size);
    if self_check {
        let singles = evaluate(&layers, &digits, &validation, 1);
        let partial = evaluate(&layers, &digits, &validation, 3);
        assert_eq!(initial.2, singles.2);
        assert_eq!(initial.2, partial.2);
        assert!((initial.0 - singles.0).abs() < 1e-5 && (initial.0 - partial.0).abs() < 1e-5);
    }

    println!(
        "epoch=0 validation_loss={:.5} validation_accuracy={:.4}",
        initial.0, initial.1
    );
    let mut best = layers.clone();
    let mut best_loss = initial.0;
    let mut best_epoch = 0;
    let mut step = 0i32;
    // 4. Actual training and backward pass, directly in main.
    for epoch in 1..=epochs {
        shuffle(&mut training, &mut random_state);
        let mut epoch_loss = 0.0;
        for examples in training.chunks(batch_size) {
            let (input, labels) = batch(&digits, examples);
            let (states, _, _) = forward(&layers, &input);
            let (loss, gradient) = cross_entropy(states.last().unwrap(), &labels);
            if !loss.is_finite() {
                return Err("Ошибка обучения не конечна; уменьши learning-rate".into());
            }
            let (dh, ow, ob) = dense_backward(&states[1], &layers[1].0, &gradient);
            let (_, hw, hb) =
                dense_backward(&states[0], &layers[0].0, &relu_backward(&states[1], &dh));
            let gradients = vec![(hw, hb), (ow, ob)];
            // Compute every gradient before changing any layer. Adam for weights and biases.
            step += 1;
            let correction1 = 1.0 - 0.9f32.powi(step);
            let correction2 = 1.0 - 0.999f32.powi(step);
            for i in 0..layers.len() {
                for (((parameter, m), v), &g) in layers[i]
                    .0
                    .iter_mut()
                    .zip(first_moments[i].0.iter_mut())
                    .zip(second_moments[i].0.iter_mut())
                    .zip(gradients[i].0.iter())
                {
                    *m = 0.9 * *m + 0.1 * g;
                    *v = 0.999 * *v + 0.001 * g * g;
                    *parameter -=
                        learning_rate * (*m / correction1) / ((*v / correction2).sqrt() + 1e-8);
                }
                for (((parameter, m), v), &g) in layers[i]
                    .1
                    .iter_mut()
                    .zip(first_moments[i].1.iter_mut())
                    .zip(second_moments[i].1.iter_mut())
                    .zip(gradients[i].1.iter())
                {
                    *m = 0.9 * *m + 0.1 * g;
                    *v = 0.999 * *v + 0.001 * g * g;
                    *parameter -=
                        learning_rate * (*m / correction1) / ((*v / correction2).sqrt() + 1e-8);
                }
            }
            epoch_loss += loss * examples.len() as f32;
        }
        let metrics = evaluate(&layers, &digits, &validation, batch_size);
        if !metrics.0.is_finite() {
            return Err("Validation loss не конечна".into());
        }
        if metrics.0 < best_loss {
            best_loss = metrics.0;
            best_epoch = epoch;
            best = layers.clone();
        }
        println!(
            "epoch={epoch} train_loss={:.5} validation_loss={:.5} validation_accuracy={:.4} elapsed={:.1}s",
            epoch_loss / training.len() as f32,
            metrics.0,
            metrics.1,
            start.elapsed().as_secs_f32()
        );
    }
    if self_check {
        assert!(
            best_loss < initial.0 * 0.8,
            "synthetic loss must decrease: {} -> {best_loss}",
            initial.0
        );
        for (before, after) in initial_layers.iter().zip(&layers) {
            assert_ne!(before.0, after.0, "every layer must learn");
        }
        println!("Самопроверка: производные и обучение всех слоёв прошли");
        return Ok(());
    }
    // 5. Read official test only after choosing the epoch with validation loss.
    let test = load_digits(&data.join("test"))?;
    let test_indices: Vec<_> = (0..test.len()).collect();
    let metrics = evaluate(&best, &test, &test_indices, batch_size);
    let baseline =
        test.iter().filter(|d| d.1 as usize == majority).count() as f32 / test.len() as f32;
    println!(
        "selected_epoch={best_epoch}; test={} loss={:.5} accuracy={:.4} baseline={baseline:.4} elapsed={:.1}s",
        test.len(),
        metrics.0,
        metrics.1,
        start.elapsed().as_secs_f32()
    );
    println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
    for row in metrics.2 {
        println!("{row:?}");
    }
    Ok(())
}
