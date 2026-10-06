fn main() -> Result<(), String> {
    // Урок 243. Путь данных: PNG -> 784 числа -> средние изображения -> прогноз цифры.
    // Каждая картинка имеет размер 28×28, поэтому в ней 28*28 = 784 пикселя.
    // У одной записи два поля: массив яркостей и правильная метка (цифра 0–9).
    // «Обучение» здесь — накопление и усреднение примеров каждой цифры.
    // Затем новую картинку сравниваем с десятью средними и выбираем ближайшее.
    //
    // Читай блоки последовательно: настройки, подготовка/обучение, validation, test.
    // Train учит модель; validation проверяет её на отложенных примерах;
    // test даёт итоговую независимую оценку. Ниже средние вычисляются только по train.
    // Фигурные скобки ограничивают жизнь временных имён; последняя строка блока
    // без точки с запятой возвращает результат следующему этапу.

    // Настройки: парсер и флаг задания не выходят из этого блока.
    let (data, self_check) = {
        let mut data =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
        let mut self_check = false;
        let mut exercise = false;
        // Настройки можно переопределить после -- в команде cargo run.
        // skip(1) пропускает имя программы. Этот итератор нужен только разбору аргументов.
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
        // Самостоятельный вопрос отделён от автоматических проверок.
        // Впиши рассчитанный ответ в Some(...); этот режим сразу завершит программу.
        if exercise {
            // Самостоятельное задание: Чему равен нормализованный пиксель 255?
            let answer: Option<f32> = None;
            assert_eq!(
                answer.expect("реши вопрос и замени None на Some(ответ)"),
                1.0
            );
            return Ok(());
        }
        (data, self_check)
    };
    // Обучение и validation: наружу выходят только готовые средние и baseline.
    let (means, majority) = {
        let digits = {
            // Загрузчик возвращает записи (784 нормализованных пикселя, правильная цифра).
            // Он локален блоку загрузки: после получения данных это имя больше не нужно.
            let load_digits =
                |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
                    let mut digits = Vec::new();
                    // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                    // Имя самого PNG — его индекс, а не ответ классификатора.
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
                        // Файловая система не обещает порядок чтения. Сортировка фиксирует порядок
                        // картинок и, следовательно, одинаковое разделение train/validation при повторном запуске.
                        paths.sort();
                        if paths.is_empty() {
                            return Err(format!("{}: нет PNG", class.display()));
                        }
                        for path in paths {
                            // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                            // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                            let decoded = (|| -> Result<[f64; 784], String> {
                                let file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
                                let mut reader = png::Decoder::new(std::io::BufReader::new(file))
                                    .read_info()
                                    .map_err(|e| e.to_string())?;
                                let info = reader.info();
                                // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                                // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                                if info.width != 28
                                    || info.height != 28
                                    || info.color_type != png::ColorType::Grayscale
                                    || info.bit_depth != png::BitDepth::Eight
                                    || info.animation_control.is_some()
                                {
                                    return Err(
                                        "Ожидается статический PNG 28×28, grayscale, 8 бит".into(),
                                    );
                                }
                                // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                                // Пиксели идут строка за строкой: индекс y*28+x соответствует координатам (y,x).
                                let mut pixels = [0u8; 784];
                                reader.next_frame(&mut pixels).map_err(|e| e.to_string())?;
                                reader.finish().map_err(|e| e.to_string())?;
                                // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                                // Это фиксированная нормализация, не требующая статистик validation или test.
                                Ok(std::array::from_fn(|i| f64::from(pixels[i]) / 255.0))
                            })()
                            .map_err(|e| format!("{}: {e}", path.display()))?;
                            digits.push((decoded, label));
                        }
                    }
                    Ok(digits)
                };
            if self_check {
                // Проверяем чтение PNG на временных картинках, включая неверные входы.
                // Эта ветка проверяет загрузчик на временных PNG, а не использует настоящий MNIST.
                // Уникальное имя предотвращает столкновения нескольких запусков проверки.
                let unique = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| e.to_string())?
                    .as_nanos();
                let temporary = std::env::temp_dir()
                    .join(format!("mnist-main-{}-{unique}", std::process::id()));
                // Проверяем настоящую цепочку чтения: метки папок, сортировку и нормализацию,
                // затем намеренно испорченный файл, неверный размер и RGB вместо оттенков серого.
                // Результат временно сохраняем, чтобы сначала удалить файлы даже при обычной ошибке.
                let checked = (|| -> Result<(), String> {
                    assert!(load_digits(&temporary).is_err());
                    // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                    // Имя самого PNG — его индекс, а не ответ классификатора.
                    for label in 0..10u8 {
                        let class = temporary.join(label.to_string());
                        std::fs::create_dir_all(&class).map_err(|e| e.to_string())?;
                        for (name, value) in [("00002.png", 255u8), ("00001.png", 0u8)] {
                            let file = std::fs::File::create(class.join(name))
                                .map_err(|e| e.to_string())?;
                            let mut encoder = png::Encoder::new(file, 28, 28);
                            encoder.set_color(png::ColorType::Grayscale);
                            encoder.set_depth(png::BitDepth::Eight);
                            let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
                            writer
                                .write_image_data(&[value; 784])
                                .map_err(|e| e.to_string())?;
                            writer.finish().map_err(|e| e.to_string())?;
                        }
                        std::fs::write(class.join("notes.txt"), "ignored")
                            .map_err(|e| e.to_string())?;
                    }
                    // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                    // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
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
                (0..10u8)
                    .flat_map(|label| (0..5).map(move |_| ([f64::from(label) / 10.0; 784], label)))
                    .collect()
            } else {
                load_digits(&data.join("train"))?
            }
        };
        let (means, majority, validation) = {
            // Для каждой цифры считаем только обучающие примеры.
            // Эти числа нужны для деления суммы пикселей на число картинок и выбора baseline.
            let mut counts = [0usize; 10];
            // sums[c][p] — сумма яркости пикселя p у всех train-картинок цифры c.
            // После деления получим десять средних изображений, каждое из 784 чисел.
            let mut sums = [[0.0; 784]; 10];
            // seen считает все просмотренные примеры класса, включая отложенные.
            // Он отвечает за разделение, а counts — только за количество использованных в обучении.
            let mut seen = [0usize; 10];
            let mut validation = Vec::new();
            for (index, (pixels, label)) in digits.iter().enumerate() {
                let class = *label as usize;
                // Отложенный пример добавляем только в validation: он не меняет sums или counts.
                // Так оценка проверяет перенос на картинки, которых среднее изображение не видело.
                if seen[class] % 5 == 0 {
                    validation.push(index);
                } else {
                    counts[class] += 1;
                    for (sum, pixel) in sums[class].iter_mut().zip(pixels) {
                        // Прибавляем яркость в ТОЙ ЖЕ координате. Это выравнивание всех картинок по пикселям:
                        // сдвинутые или очень разные почерки будут размываться в среднем изображении.
                        *sum += pixel;
                    }
                }
                seen[class] += 1;
            }
            if counts.contains(&0) {
                return Err("В train нужны все десять классов".into());
            }
            // Делим покоординатную сумму на число примеров класса: mean[c,p]=sum[c,p]/count[c].
            // Обучение этой простой модели на этом заканчивается — итераций и градиентов здесь нет.
            let mut means = sums;
            for (mean, count) in means.iter_mut().zip(counts) {
                for pixel in mean {
                    *pixel /= count as f64;
                }
            }
            // Baseline — постоянный прогноз самой частой цифры train. Он показывает,
            // насколько модель лучше простого ответа без анализа пикселей.
            // При равной частоте выбираем меньшую цифру; метки test в выборе не участвуют.
            let majority = (0..10)
                .max_by_key(|&i| (counts[i], std::cmp::Reverse(i)))
                .unwrap();
            println!(
                "train: n={}, baseline digit={majority}; split: every fifth per class",
                counts.iter().sum::<usize>()
            );
            if self_check {
                assert_eq!(counts, [4; 10]);
            }
            (means, majority, validation)
        };
        // Временные индексы и метрики validation локальны этой оценке.
        {
            let name = "validation";
            let records = &digits;
            let indices = validation;

            // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
            // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
            let mut confusion = [[0usize; 10]; 10];
            let mut baseline = 0usize;
            for &index in &indices {
                let (pixels, label) = &records[index];
                // Проверяем все десять классов и выбираем наименьшее расстояние до среднего.
                // Метка текущей картинки не участвует в выборе; её используем позже для проверки ответа.
                let predicted = (0..10)
                    .min_by(|&a, &b| {
                        // Квадрат евклидова расстояния: sum_p((pixel[p]-mean[c,p])²).
                        // Квадраты не дают положительным и отрицательным различиям сократиться.
                        // Корень не нужен: он не меняет порядок расстояний и выбранную цифру.
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
            // Сумма диагонали матрицы ошибок — число правильных прогнозов.
            // Accuracy = correct / число проверенных картинок; например, 8 верных из 10 дают 0.8.
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
            }
            if name == "test" {
                println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
                for row in confusion {
                    println!("{row:?}");
                }
            }
        }
        (means, majority)
    };
    // Test загружается после обучения; его данные и метрики остаются здесь.
    {
        let test = {
            // Загрузчик возвращает записи (784 нормализованных пикселя, правильная цифра).
            // Он локален блоку загрузки: после получения данных это имя больше не нужно.
            let load_digits =
                |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
                    let mut digits = Vec::new();
                    // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                    // Имя самого PNG — его индекс, а не ответ классификатора.
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
                        // Файловая система не обещает порядок чтения. Сортировка фиксирует порядок
                        // картинок и, следовательно, одинаковое разделение train/validation при повторном запуске.
                        paths.sort();
                        if paths.is_empty() {
                            return Err(format!("{}: нет PNG", class.display()));
                        }
                        for path in paths {
                            // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                            // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                            let decoded = (|| -> Result<[f64; 784], String> {
                                let file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
                                let mut reader = png::Decoder::new(std::io::BufReader::new(file))
                                    .read_info()
                                    .map_err(|e| e.to_string())?;
                                let info = reader.info();
                                // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                                // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                                if info.width != 28
                                    || info.height != 28
                                    || info.color_type != png::ColorType::Grayscale
                                    || info.bit_depth != png::BitDepth::Eight
                                    || info.animation_control.is_some()
                                {
                                    return Err(
                                        "Ожидается статический PNG 28×28, grayscale, 8 бит".into(),
                                    );
                                }
                                // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                                // Пиксели идут строка за строкой: индекс y*28+x соответствует координатам (y,x).
                                let mut pixels = [0u8; 784];
                                reader.next_frame(&mut pixels).map_err(|e| e.to_string())?;
                                reader.finish().map_err(|e| e.to_string())?;
                                // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                                // Это фиксированная нормализация, не требующая статистик validation или test.
                                Ok(std::array::from_fn(|i| f64::from(pixels[i]) / 255.0))
                            })()
                            .map_err(|e| format!("{}: {e}", path.display()))?;
                            digits.push((decoded, label));
                        }
                    }
                    Ok(digits)
                };
            if self_check {
                (0..10u8)
                    .flat_map(|label| (0..5).map(move |_| ([f64::from(label) / 10.0; 784], label)))
                    .collect()
            } else {
                load_digits(&data.join("test"))?
            }
        };
        let name = "test";
        let records = &test;
        let indices: Vec<_> = (0..test.len()).collect();

        // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
        // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
        let mut confusion = [[0usize; 10]; 10];
        let mut baseline = 0usize;
        for &index in &indices {
            let (pixels, label) = &records[index];
            // Проверяем все десять классов и выбираем наименьшее расстояние до среднего.
            // Метка текущей картинки не участвует в выборе; её используем позже для проверки ответа.
            let predicted = (0..10)
                .min_by(|&a, &b| {
                    // Квадрат евклидова расстояния: sum_p((pixel[p]-mean[c,p])²).
                    // Квадраты не дают положительным и отрицательным различиям сократиться.
                    // Корень не нужен: он не меняет порядок расстояний и выбранную цифру.
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
        // Сумма диагонали матрицы ошибок — число правильных прогнозов.
        // Accuracy = correct / число проверенных картинок; например, 8 верных из 10 дают 0.8.
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
