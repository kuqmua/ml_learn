fn main() -> Result<(), String> {
    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя или метка цифры;
    // u64 — состояние генератора; bool — флаг; String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Option<T> — значение или None; Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 — дробное число двойной точности; средние: [[f64; 784]; 10].

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
    let (data, self_check): (std::path::PathBuf, bool) = {
        let mut data: std::path::PathBuf =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
        let mut self_check: bool = false;
        let mut exercise: bool = false;
        // Настройки можно переопределить после -- в команде cargo run.
        // skip(1) пропускает имя программы. Этот итератор нужен только разбору аргументов.
        let mut arguments: std::iter::Skip<std::env::Args> = std::env::args().skip(1);
        // Типы переменных: key: String.
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
    let (means, majority): ([[f64; 784]; 10], usize) = {
        let digits: Vec<([f64; 784], u8)> = {
            // Загрузчик возвращает записи (784 нормализованных пикселя, правильная цифра).
            // Он локален блоку загрузки: после получения данных это имя больше не нужно.

            let load_digits =
                |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
                    let mut digits: Vec<([f64; 784], u8)> = Vec::new();
                    // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                    // Имя самого PNG — его индекс, а не ответ классификатора.
                    // Типы переменных: label: u8.
                    for label in 0..10u8 {
                        let class: std::path::PathBuf = directory.join(label.to_string());
                        let entries: std::fs::ReadDir =
                            std::fs::read_dir(&class).map_err(|e: std::io::Error| -> String {
                                format!(
                                    "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                                    class.display()
                                )
                            })?;
                        let mut paths: Vec<std::path::PathBuf> = Vec::new();
                        // Типы переменных: entry: Result<std::fs::DirEntry, std::io::Error>.
                        for entry in entries {
                            let path: std::path::PathBuf = entry
                                .map_err(|e: std::io::Error| -> String { e.to_string() })?
                                .path();
                            if path
                                .extension()
                                .is_some_and(|ext: &std::ffi::OsStr| -> bool {
                                    ext.eq_ignore_ascii_case("png")
                                })
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
                        // Типы переменных: path: std::path::PathBuf.
                        for path in paths {
                            // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                            // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                            let decoded: [f64; 784] = (|| -> Result<[f64; 784], String> {
                                let file: std::fs::File = std::fs::File::open(&path)
                                    .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                                let mut reader: png::Reader<std::io::BufReader<std::fs::File>> =
                                    png::Decoder::new(std::io::BufReader::new(file))
                                        .read_info()
                                        .map_err(|e: png::DecodingError| -> String {
                                            e.to_string()
                                        })?;
                                let info: &png::Info<'_> = reader.info();
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
                                let mut pixels: [u8; 784] = [0u8; 784];
                                reader
                                    .next_frame(&mut pixels)
                                    .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                                reader
                                    .finish()
                                    .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                                // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                                // Это фиксированная нормализация, не требующая статистик validation или test.
                                Ok(std::array::from_fn(|i: usize| -> f64 {
                                    f64::from(pixels[i]) / 255.0
                                }))
                            })()
                            .map_err(|e: String| -> String {
                                format!("{}: {e}", path.display())
                            })?;
                            digits.push((decoded, label));
                        }
                    }
                    Ok(digits)
                };
            if self_check {
                // Проверяем чтение PNG на временных картинках, включая неверные входы.
                // Эта ветка проверяет загрузчик на временных PNG, а не использует настоящий MNIST.
                // Уникальное имя предотвращает столкновения нескольких запусков проверки.
                let unique: u128 = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e: std::time::SystemTimeError| -> String { e.to_string() })?
                    .as_nanos();
                let temporary: std::path::PathBuf = std::env::temp_dir()
                    .join(format!("mnist-main-{}-{unique}", std::process::id()));
                // Проверяем настоящую цепочку чтения: метки папок, сортировку и нормализацию,
                // затем намеренно испорченный файл, неверный размер и RGB вместо оттенков серого.
                // Результат временно сохраняем, чтобы сначала удалить файлы даже при обычной ошибке.
                let checked: Result<(), String> = (|| -> Result<(), String> {
                    assert!(load_digits(&temporary).is_err());
                    // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                    // Имя самого PNG — его индекс, а не ответ классификатора.
                    // Типы переменных: label: u8.
                    for label in 0..10u8 {
                        let class: std::path::PathBuf = temporary.join(label.to_string());
                        std::fs::create_dir_all(&class)
                            .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                        // Типы переменных: name: &str, value: u8.
                        for (name, value) in [("00002.png", 255u8), ("00001.png", 0u8)] {
                            let file: std::fs::File = std::fs::File::create(class.join(name))
                                .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                            let mut encoder: png::Encoder<'_, std::fs::File> =
                                png::Encoder::new(file, 28, 28);
                            encoder.set_color(png::ColorType::Grayscale);
                            encoder.set_depth(png::BitDepth::Eight);
                            let mut writer: png::Writer<std::fs::File> = encoder
                                .write_header()
                                .map_err(|e: png::EncodingError| -> String { e.to_string() })?;
                            writer
                                .write_image_data(&[value; 784])
                                .map_err(|e: png::EncodingError| -> String { e.to_string() })?;
                            writer
                                .finish()
                                .map_err(|e: png::EncodingError| -> String { e.to_string() })?;
                        }
                        std::fs::write(class.join("notes.txt"), "ignored")
                            .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                    }
                    // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                    // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                    let decoded: Vec<([f64; 784], u8)> = load_digits(&temporary)?;
                    assert_eq!(decoded.len(), 20);
                    // Типы переменных: i: usize, pixels: &[f64; 784], label: &u8.
                    for (i, (pixels, label)) in decoded.iter().enumerate() {
                        assert_eq!(*label as usize, i / 2);
                        assert_eq!(*pixels, [(i % 2) as f64; 784]);
                    }
                    let path: std::path::PathBuf = temporary.join("0/00001.png");
                    std::fs::write(&path, b"broken PNG")
                        .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                    assert!(load_digits(&temporary).err().unwrap().contains("00001.png"));
                    // Типы переменных: width: u32, color: png::ColorType, channels: usize.
                    for (width, color, channels) in [
                        (27, png::ColorType::Grayscale, 1),
                        (28, png::ColorType::Rgb, 3),
                    ] {
                        let file: std::fs::File = std::fs::File::create(&path)
                            .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                        let mut encoder: png::Encoder<'_, std::fs::File> =
                            png::Encoder::new(file, width, 28);
                        encoder.set_color(color);
                        encoder.set_depth(png::BitDepth::Eight);
                        let mut writer: png::Writer<std::fs::File> = encoder
                            .write_header()
                            .map_err(|e: png::EncodingError| -> String { e.to_string() })?;
                        writer
                            .write_image_data(&vec![0; width as usize * 28 * channels])
                            .map_err(|e: png::EncodingError| -> String { e.to_string() })?;
                        writer
                            .finish()
                            .map_err(|e: png::EncodingError| -> String { e.to_string() })?;
                        assert!(load_digits(&temporary).is_err());
                    }
                    Ok(())
                })();
                if temporary.exists() {
                    std::fs::remove_dir_all(&temporary)
                        .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                }
                checked?;
            }
            if self_check {
                (0..10u8)
                    .flat_map(|label: u8| {
                        // Результат — итератор с элементами ([f64; 784], u8).
                        // Его тип включает анонимное замыкание map, поэтому выводится компилятором.
                        (0..5).map(move |_: i32| -> ([f64; 784], u8) {
                            ([f64::from(label) / 10.0; 784], label)
                        })
                    })
                    .collect()
            } else {
                load_digits(&data.join("train"))?
            }
        };
        let (means, majority, validation): ([[f64; 784]; 10], usize, Vec<usize>) = {
            // Для каждой цифры считаем только обучающие примеры.
            // Эти числа нужны для деления суммы пикселей на число картинок и выбора baseline.
            let mut counts: [usize; 10] = [0usize; 10];
            // sums[c][p] — сумма яркости пикселя p у всех train-картинок цифры c.
            // После деления получим десять средних изображений, каждое из 784 чисел.
            let mut sums: [[f64; 784]; 10] = [[0.0; 784]; 10];
            // seen считает все просмотренные примеры класса, включая отложенные.
            // Он отвечает за разделение, а counts — только за количество использованных в обучении.
            let mut seen: [usize; 10] = [0usize; 10];
            let mut validation: Vec<usize> = Vec::new();
            // Типы переменных: index: usize, pixels: &[f64; 784], label: &u8.
            for (index, (pixels, label)) in digits.iter().enumerate() {
                let class: usize = *label as usize;
                // Отложенный пример добавляем только в validation: он не меняет sums или counts.
                // Так оценка проверяет перенос на картинки, которых среднее изображение не видело.
                if seen[class] % 5 == 0 {
                    validation.push(index);
                } else {
                    counts[class] += 1;
                    // Типы переменных: sum: &mut f64, pixel: &f64.
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
            let mut means: [[f64; 784]; 10] = sums;
            // Типы переменных: mean: &mut [f64; 784], count: usize.
            for (mean, count) in means.iter_mut().zip(counts) {
                // Типы переменных: pixel: &mut f64.
                for pixel in mean {
                    *pixel /= count as f64;
                }
            }
            // Baseline — постоянный прогноз самой частой цифры train. Он показывает,
            // насколько модель лучше простого ответа без анализа пикселей.
            // При равной частоте выбираем меньшую цифру; метки test в выборе не участвуют.
            let majority: usize = (0..10)
                .max_by_key(|&i: &usize| -> (usize, std::cmp::Reverse<usize>) {
                    (counts[i], std::cmp::Reverse(i))
                })
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
            let name: &str = "validation";
            let records: &Vec<([f64; 784], u8)> = &digits;
            let indices: Vec<usize> = validation;

            // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
            // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
            let mut confusion: [[usize; 10]; 10] = [[0usize; 10]; 10];
            let mut baseline: usize = 0usize;
            // Типы переменных: index: usize.
            for &index in &indices {
                let (pixels, label): &([f64; 784], u8) = &records[index];
                // Проверяем все десять классов и выбираем наименьшее расстояние до среднего.
                // Метка текущей картинки не участвует в выборе; её используем позже для проверки ответа.
                let predicted: usize = (0..10)
                    .min_by(|&a: &usize, &b: &usize| -> std::cmp::Ordering {
                        // Квадрат евклидова расстояния: sum_p((pixel[p]-mean[c,p])²).
                        // Квадраты не дают положительным и отрицательным различиям сократиться.
                        // Корень не нужен: он не меняет порядок расстояний и выбранную цифру.

                        let distance = |class: usize| -> f64 {
                            pixels
                                .iter()
                                .zip(means[class])
                                .map(|(x, y): (&f64, f64)| -> f64 { (x - y).powi(2) })
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
            let correct: usize = (0..10).map(|i: usize| -> usize { confusion[i][i] }).sum();
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
                // Типы переменных: row: [usize; 10].
                for row in confusion {
                    println!("{row:?}");
                }
            }
        }
        (means, majority)
    };
    // Test загружается после обучения; его данные и метрики остаются здесь.
    {
        let test: Vec<([f64; 784], u8)> = {
            // Загрузчик возвращает записи (784 нормализованных пикселя, правильная цифра).
            // Он локален блоку загрузки: после получения данных это имя больше не нужно.

            let load_digits =
                |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
                    let mut digits: Vec<([f64; 784], u8)> = Vec::new();
                    // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                    // Имя самого PNG — его индекс, а не ответ классификатора.
                    // Типы переменных: label: u8.
                    for label in 0..10u8 {
                        let class: std::path::PathBuf = directory.join(label.to_string());
                        let entries: std::fs::ReadDir =
                            std::fs::read_dir(&class).map_err(|e: std::io::Error| -> String {
                                format!(
                                    "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                                    class.display()
                                )
                            })?;
                        let mut paths: Vec<std::path::PathBuf> = Vec::new();
                        // Типы переменных: entry: Result<std::fs::DirEntry, std::io::Error>.
                        for entry in entries {
                            let path: std::path::PathBuf = entry
                                .map_err(|e: std::io::Error| -> String { e.to_string() })?
                                .path();
                            if path
                                .extension()
                                .is_some_and(|ext: &std::ffi::OsStr| -> bool {
                                    ext.eq_ignore_ascii_case("png")
                                })
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
                        // Типы переменных: path: std::path::PathBuf.
                        for path in paths {
                            // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                            // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                            let decoded: [f64; 784] = (|| -> Result<[f64; 784], String> {
                                let file: std::fs::File = std::fs::File::open(&path)
                                    .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                                let mut reader: png::Reader<std::io::BufReader<std::fs::File>> =
                                    png::Decoder::new(std::io::BufReader::new(file))
                                        .read_info()
                                        .map_err(|e: png::DecodingError| -> String {
                                            e.to_string()
                                        })?;
                                let info: &png::Info<'_> = reader.info();
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
                                let mut pixels: [u8; 784] = [0u8; 784];
                                reader
                                    .next_frame(&mut pixels)
                                    .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                                reader
                                    .finish()
                                    .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                                // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                                // Это фиксированная нормализация, не требующая статистик validation или test.
                                Ok(std::array::from_fn(|i: usize| -> f64 {
                                    f64::from(pixels[i]) / 255.0
                                }))
                            })()
                            .map_err(|e: String| -> String {
                                format!("{}: {e}", path.display())
                            })?;
                            digits.push((decoded, label));
                        }
                    }
                    Ok(digits)
                };
            if self_check {
                (0..10u8)
                    .flat_map(|label: u8| {
                        // Результат — итератор с элементами ([f64; 784], u8).
                        // Его тип включает анонимное замыкание map, поэтому выводится компилятором.
                        (0..5).map(move |_: i32| -> ([f64; 784], u8) {
                            ([f64::from(label) / 10.0; 784], label)
                        })
                    })
                    .collect()
            } else {
                load_digits(&data.join("test"))?
            }
        };
        let name: &str = "test";
        let records: &Vec<([f64; 784], u8)> = &test;
        let indices: Vec<_> = (0..test.len()).collect();

        // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
        // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
        let mut confusion: [[usize; 10]; 10] = [[0usize; 10]; 10];
        let mut baseline: usize = 0usize;
        // Типы переменных: index: usize.
        for &index in &indices {
            let (pixels, label): &([f64; 784], u8) = &records[index];
            // Проверяем все десять классов и выбираем наименьшее расстояние до среднего.
            // Метка текущей картинки не участвует в выборе; её используем позже для проверки ответа.
            let predicted: usize = (0..10)
                .min_by(|&a: &usize, &b: &usize| -> std::cmp::Ordering {
                    // Квадрат евклидова расстояния: sum_p((pixel[p]-mean[c,p])²).
                    // Квадраты не дают положительным и отрицательным различиям сократиться.
                    // Корень не нужен: он не меняет порядок расстояний и выбранную цифру.

                    let distance = |class: usize| -> f64 {
                        pixels
                            .iter()
                            .zip(means[class])
                            .map(|(x, y): (&f64, f64)| -> f64 { (x - y).powi(2) })
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
        let correct: usize = (0..10).map(|i: usize| -> usize { confusion[i][i] }).sum();
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
            // Типы переменных: row: [usize; 10].
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
