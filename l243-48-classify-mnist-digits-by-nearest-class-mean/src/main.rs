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
    let mut data =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
    let mut self_check = false;
    let mut exercise = false;
    let mut arguments = std::env::args().skip(1);
    while let Some(key) = arguments.next() {
        match key.as_str() {
            "--self-check" => self_check = true,
            "--exercise" => exercise = true,
            "--data" => data = arguments.next().ok_or("Нет значения для --data")?.into(),
            "--help" => {
                println!("--data PNG_ROOT --self-check --exercise");
                return Ok(());
            }
            _ => return Err(format!("Неизвестный аргумент {key}")),
        }
    }
    if exercise {
        // Самостоятельное задание: Чему равен нормализованный пиксель 255?
        let answer: Option<f32> = None;
        assert_eq!(
            answer.expect("реши вопрос и замени None на Some(ответ)"),
            1.0
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
    let digits = if self_check {
        (0..10u8)
            .flat_map(|label| (0..5).map(move |_| ([f64::from(label) / 10.0; 784], label)))
            .collect()
    } else {
        load_digits(&data.join("train"))?
    };
    let mut counts = [0usize; 10];
    let mut sums = [[0.0; 784]; 10];
    let mut seen = [0usize; 10];
    let mut validation = Vec::new();
    // Every fifth example per class is held out before computing means.
    for (index, (pixels, label)) in digits.iter().enumerate() {
        let class = *label as usize;
        if seen[class] % 5 == 0 {
            validation.push(index);
        } else {
            counts[class] += 1;
            for (sum, pixel) in sums[class].iter_mut().zip(pixels) {
                *sum += pixel;
            }
        }
        seen[class] += 1;
    }
    if counts.contains(&0) {
        return Err("В train нужны все десять классов".into());
    }
    let mut means = sums;
    for (mean, count) in means.iter_mut().zip(counts) {
        for pixel in mean {
            *pixel /= count as f64;
        }
    }
    let majority = (0..10)
        .max_by_key(|&i| (counts[i], std::cmp::Reverse(i)))
        .unwrap();
    println!(
        "train: n={}, baseline digit={majority}; split: every fifth per class",
        counts.iter().sum::<usize>()
    );
    let test = if self_check {
        digits.clone()
    } else {
        load_digits(&data.join("test"))?
    };
    // Evaluate validation and official test using only training means.
    for (name, records, indices) in [
        ("validation", &digits, validation),
        ("test", &test, (0..test.len()).collect()),
    ] {
        let mut confusion = [[0usize; 10]; 10];
        let mut baseline = 0usize;
        for &index in &indices {
            let (pixels, label) = &records[index];
            let predicted = (0..10)
                .min_by(|&a, &b| {
                    let distance = |class: usize| {
                        pixels
                            .iter()
                            .zip(means[class])
                            .map(|(x, y)| (x - y).powi(2))
                            .sum::<f64>()
                    };
                    distance(a).total_cmp(&distance(b))
                })
                .unwrap();
            confusion[*label as usize][predicted] += 1;
            baseline += usize::from(*label as usize == majority);
        }
        let correct: usize = (0..10).map(|i| confusion[i][i]).sum();
        println!(
            "{name}: n={}, accuracy={:.4}, baseline={:.4}",
            indices.len(),
            correct as f64 / indices.len() as f64,
            baseline as f64 / indices.len() as f64
        );
        if self_check {
            assert_eq!(correct, indices.len());
            assert_eq!(majority, 0);
            assert_eq!(counts, [4; 10]);
        }
        if name == "test" {
            println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
            for row in confusion {
                println!("{row:?}");
            }
        }
    }
    if self_check {
        println!("Самопроверка средних изображений прошла");
    }
    Ok(())
}
