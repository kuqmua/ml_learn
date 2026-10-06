fn main() -> Result<(), String> {
    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя или метка цифры;
    // String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
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

    let data: std::path::PathBuf =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");

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

            load_digits(&data.join("train"))?
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
            load_digits(&data.join("test"))?
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

        println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
        // Типы переменных: row: [usize; 10].
        for row in confusion {
            println!("{row:?}");
        }
    }

    Ok(())
}
