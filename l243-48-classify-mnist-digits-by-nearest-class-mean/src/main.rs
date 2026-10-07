fn main() -> Result<(), String> {
    // Урок 243. Путь данных: PNG -> 784 числа -> средние изображения -> прогноз цифры.
    // Каждая картинка имеет размер 28×28, поэтому в ней 28*28 = 784 пикселя.
    // У одной записи два поля: массив яркостей и правильная метка (цифра 0–9).
    // «Обучение» здесь — накопление и усреднение примеров каждой цифры.
    // Затем новую картинку сравниваем с десятью средними и выбираем ближайшее.
    //
    // Сначала объявлены Digit и общие операции: загрузка PNG, оценка и вывод метрик.
    // Затем выполняются обучение средних, оценка validation и оценка test.
    // Train учит модель; validation проверяет её на отложенных примерах;
    // test даёт итоговую независимую оценку. Ниже средние вычисляются только по train.
    // Фигурные скобки ограничивают жизнь временных имён; последняя строка блока
    // без точки с запятой возвращает результат следующему этапу.

    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя; Digit — метка цифры;
    // String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 — дробное число двойной точности; DataByDigit хранит отдельное поле для каждой цифры.

    // Метка — одна из десяти цифр. Значения вроде 42 теперь нельзя записать как метку.
    // digit as usize даёт число 0..9 для индексов массивов, имён папок и печати.
    // Copy позволяет копировать метки; PartialEq/Eq — сравнивать правильную цифру с прогнозом.
    // repr(u8) задаёт однобайтовое хранение варианта, но тип метки остаётся Digit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    enum Digit {
        Zero = 0,
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
    }
    // У каждой цифры своё именованное поле. T задаёт содержимое поля:
    // usize для счётчика, [f64; 784] для суммы яркостей или среднего изображения.
    struct DataByDigit<T> {
        zero: T,
        one: T,
        two: T,
        three: T,
        four: T,
        five: T,
        six: T,
        seven: T,
        eight: T,
        nine: T,
    }

    // Порядок соответствует папкам 0..9; при равном расстоянии выбирается меньшая цифра.
    let all_digits: [Digit; 10] = [
        Digit::Zero,
        Digit::One,
        Digit::Two,
        Digit::Three,
        Digit::Four,
        Digit::Five,
        Digit::Six,
        Digit::Seven,
        Digit::Eight,
        Digit::Nine,
    ];

    // Корень PNG фиксирован относительно папки урока; аргументы запуска не требуются.
    let dataset_directory: std::path::PathBuf =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");

    // Запись датасета: ([f64; 784], Digit), например (normalized_pixels, Digit::Seven).
    // Один загрузчик PNG для train и test: возвращает пиксели и правильные метки.
    let load_labeled_png_images =
        |images_directory: &std::path::Path| -> Result<Vec<([f64; 784], Digit)>, String> {
            let mut labeled_images: Vec<([f64; 784], Digit)> = Vec::new();
            // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
            // Имя PNG обозначает номер файла; правильная цифра определяется папкой.
            // Типы переменных: digit: Digit.
            for digit in all_digits {
                let digit_directory: std::path::PathBuf =
                    images_directory.join((digit as usize).to_string());
                // Изменяемый список нужен только сбору и сортировке; дальше png_file_paths неизменяемый.
                let png_file_paths: Vec<std::path::PathBuf> = {
                    let directory_entries: std::fs::ReadDir = std::fs::read_dir(&digit_directory)
                        .map_err(
                        |error: std::io::Error| -> String {
                            format!(
                                "{}: {error}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                                digit_directory.display()
                            )
                        },
                    )?;
                    let mut png_file_paths: Vec<std::path::PathBuf> = Vec::new();
                    // Типы переменных: directory_entry: Result<std::fs::DirEntry, std::io::Error>.
                    for directory_entry in directory_entries {
                        let png_file_path: std::path::PathBuf = directory_entry
                            .map_err(|error: std::io::Error| -> String { error.to_string() })?
                            .path();
                        if png_file_path.extension().is_some_and(
                            |file_extension: &std::ffi::OsStr| -> bool {
                                file_extension.eq_ignore_ascii_case("png")
                            },
                        ) {
                            png_file_paths.push(png_file_path);
                        }
                    }
                    // Файловая система не обещает порядок чтения. Сортировка фиксирует порядок
                    // картинок и, следовательно, одинаковое разделение train/validation при повторном запуске.
                    png_file_paths.sort();
                    if png_file_paths.is_empty() {
                        return Err(format!("{}: нет PNG", digit_directory.display()));
                    }
                    png_file_paths
                };
                // Типы переменных: png_file_path: std::path::PathBuf.
                for png_file_path in png_file_paths {
                    // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                    let normalized_pixels: [f64; 784] = {
                        // Читатель и изменяемый буфер живут только во время декодирования PNG.
                        // К каждой ошибке добавляем путь, чтобы найти проблемную картинку.
                        let pixel_bytes: [u8; 784] = {
                            let png_file: std::fs::File = std::fs::File::open(&png_file_path)
                                .map_err(|error: std::io::Error| -> String {
                                    format!("{}: {error}", png_file_path.display())
                                })?;
                            let mut png_reader: png::Reader<std::io::BufReader<std::fs::File>> =
                                png::Decoder::new(std::io::BufReader::new(png_file))
                                    .read_info()
                                    .map_err(|error: png::DecodingError| -> String {
                                        format!("{}: {error}", png_file_path.display())
                                    })?;
                            let png_metadata: &png::Info<'_> = png_reader.info();
                            // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                            // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                            if png_metadata.width != 28
                                || png_metadata.height != 28
                                || png_metadata.color_type != png::ColorType::Grayscale
                                || png_metadata.bit_depth != png::BitDepth::Eight
                                || png_metadata.animation_control.is_some()
                            {
                                return Err(format!(
                                    "{}: Ожидается статический PNG 28×28, grayscale, 8 бит",
                                    png_file_path.display()
                                ));
                            }
                            // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                            // Пиксели идут строка за строкой: индекс y*28+x соответствует координатам (y,x).
                            let mut pixel_bytes: [u8; 784] = [0u8; 784];
                            png_reader.next_frame(&mut pixel_bytes).map_err(
                                |error: png::DecodingError| -> String {
                                    format!("{}: {error}", png_file_path.display())
                                },
                            )?;
                            png_reader
                                .finish()
                                .map_err(|error: png::DecodingError| -> String {
                                    format!("{}: {error}", png_file_path.display())
                                })?;
                            pixel_bytes
                        };
                        // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                        // Это фиксированная нормализация, не требующая статистик validation или test.
                        std::array::from_fn(|pixel_index: usize| -> f64 {
                            f64::from(pixel_bytes[pixel_index]) / 255.0
                        })
                    };
                    labeled_images.push((normalized_pixels, digit));
                }
            }
            Ok(labeled_images)
        };

    // Проверяем распознавание выбранных картинок, не меняя средние изображения.
    // mean_image_by_digit — среднее изображение каждой цифры; labeled_images — картинки с ответами.
    // image_indices — номера проверяемых картинок; most_common_training_digit — постоянный прогноз для сравнения.
    // Результат: число верных ответов модели, число верных постоянных ответов, таблица предсказаний.
    // Доли правильных ответов вычисляет print_prediction_report.
    let evaluate_digit_predictions = |mean_image_by_digit: &DataByDigit<[f64; 784]>,
                                      labeled_images: &[([f64; 784], Digit)],
                                      image_indices: &[usize],
                                      most_common_training_digit: Digit|
     -> (usize, usize, [[usize; 10]; 10]) {
        // Счётчики меняются только во время обхода примеров.
        let (prediction_counts, constant_prediction_correct_count): ([[usize; 10]; 10], usize) = {
            // prediction_counts[истинная_цифра][предсказанная_цифра] считает такие пары.
            // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
            let mut prediction_counts: [[usize; 10]; 10] = [[0usize; 10]; 10];
            // Здесь constant_prediction_correct_count — счётчик верных ответов постоянной цифрой most_common_training_digit, а не сама цифра.
            let mut constant_prediction_correct_count: usize = 0usize;
            // Типы переменных: image_index: usize.
            for &image_index in image_indices {
                let (normalized_pixels, actual_digit): &([f64; 784], Digit) =
                    &labeled_images[image_index];
                // Проверяем все десять классов и выбираем наименьшее расстояние до среднего.
                // Метка текущей картинки не участвует в выборе; её используем позже для проверки ответа.
                let predicted_digit: Digit = all_digits
                    .into_iter()
                    .min_by(
                        |&first_candidate_digit: &Digit,
                         &second_candidate_digit: &Digit|
                         -> std::cmp::Ordering {
                            // Квадрат евклидова расстояния: sum_p((pixel_brightness[p]-mean_image[c,p])²).
                            // Квадраты не дают положительным и отрицательным различиям сократиться.
                            // Корень не нужен: он не меняет порядок расстояний и выбранную цифру.
                            let squared_distance_to_mean_image = |digit: Digit| -> f64 {
                                normalized_pixels
                                    .iter()
                                    .zip(match digit {
                                        Digit::Zero => &mean_image_by_digit.zero,
                                        Digit::One => &mean_image_by_digit.one,
                                        Digit::Two => &mean_image_by_digit.two,
                                        Digit::Three => &mean_image_by_digit.three,
                                        Digit::Four => &mean_image_by_digit.four,
                                        Digit::Five => &mean_image_by_digit.five,
                                        Digit::Six => &mean_image_by_digit.six,
                                        Digit::Seven => &mean_image_by_digit.seven,
                                        Digit::Eight => &mean_image_by_digit.eight,
                                        Digit::Nine => &mean_image_by_digit.nine,
                                    })
                                    .map(
                                        |(image_pixel_brightness, mean_pixel_brightness): (
                                            &f64,
                                            &f64,
                                        )|
                                         -> f64 {
                                            (image_pixel_brightness - mean_pixel_brightness).powi(2)
                                        },
                                    )
                                    .sum::<f64>()
                            };
                            squared_distance_to_mean_image(first_candidate_digit)
                                .total_cmp(&squared_distance_to_mean_image(second_candidate_digit))
                        },
                    )
                    .unwrap();
                // Digit преобразуем в usize только для выбора строки и столбца счётчиков.
                prediction_counts[*actual_digit as usize][predicted_digit as usize] += 1;
                constant_prediction_correct_count +=
                    usize::from(*actual_digit == most_common_training_digit);
            }
            (prediction_counts, constant_prediction_correct_count)
        };
        // Сумма диагонали матрицы ошибок — число правильных прогнозов.
        let model_correct_count: usize = (0..10)
            .map(|digit_index: usize| -> usize { prediction_counts[digit_index][digit_index] })
            .sum();
        (
            model_correct_count,
            constant_prediction_correct_count,
            prediction_counts,
        )
    };

    // Печатаем одинаковые метрики для обеих частей; матрица ошибок нужна для test.
    let print_prediction_report =
        |dataset_name: &str, evaluation_results: (usize, usize, [[usize; 10]; 10])| -> () {
            let (model_correct_count, constant_prediction_correct_count, prediction_counts): (
                usize,
                usize,
                [[usize; 10]; 10],
            ) = evaluation_results;
            // Каждая проверенная картинка увеличивает одну ячейку таблицы на 1.
            // Поэтому сумма всех ячеек равна числу проверенных картинок.
            let image_count: usize = prediction_counts.iter().flatten().sum();
            // Accuracy = model_correct_count/image_count; для постоянного прогноза аналогично. Например, 8 из 10 дают 0.8.
            println!(
                "{dataset_name}: n={image_count}, accuracy={:.4}, baseline={:.4}",
                model_correct_count as f64 / image_count as f64,
                constant_prediction_correct_count as f64 / image_count as f64
            );
            if dataset_name == "test" {
                println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
                // Типы переменных: counts_for_actual_digit: [usize; 10].
                for counts_for_actual_digit in prediction_counts {
                    println!("{counts_for_actual_digit:?}");
                }
            }
        };

    // Загружаем исходный train, отделяем validation и обучаем десять средних изображений.
    // Наружу выходят mean_image_by_digit и most_common_training_digit; картинки и счётчики остаются в этом блоке.
    let (mean_image_by_digit, most_common_training_digit): (DataByDigit<[f64; 784]>, Digit) = {
        // Загрузчик читает весь исходный train; разделение на train/validation идёт ниже.
        let labeled_images: Vec<([f64; 784], Digit)> =
            load_labeled_png_images(&dataset_directory.join("train"))?;
        // Сначала разделяем номера картинок. Сами 784 яркости остаются в labeled_images.
        let (training_image_indices, validation_image_indices): (Vec<usize>, Vec<usize>) = {
            let mut training_image_indices: Vec<usize> = Vec::new();
            let mut validation_image_indices: Vec<usize> = Vec::new();
            // Типы переменных: digit: Digit.
            for digit in all_digits {
                // Отбираем картинки одной цифры, сохраняя порядок имён файлов.
                let digit_image_indices: Vec<usize> = labeled_images
                    .iter()
                    .enumerate()
                    .filter(
                        |(_, (_, actual_digit)): &(usize, &([f64; 784], Digit))| -> bool {
                            *actual_digit == digit
                        },
                    )
                    .map(|(image_index, _): (usize, &([f64; 784], Digit))| -> usize { image_index })
                    .collect();
                // Обучение: все позиции, кроме 0, 5, 10… внутри этой цифры.
                training_image_indices.extend(
                    digit_image_indices
                        .iter()
                        .enumerate()
                        .filter(|(position, _): &(usize, &usize)| -> bool { position % 5 != 0 })
                        .map(|(_, &image_index): (usize, &usize)| -> usize { image_index }),
                );
                // Проверка: позиции 0, 5, 10…; эти картинки не участвуют в обучении.
                validation_image_indices.extend(
                    digit_image_indices
                        .iter()
                        .enumerate()
                        .filter(|(position, _): &(usize, &usize)| -> bool { position % 5 == 0 })
                        .map(|(_, &image_index): (usize, &usize)| -> usize { image_index }),
                );
            }
            (training_image_indices, validation_image_indices)
        };
        let (mean_image_by_digit, most_common_training_digit): (DataByDigit<[f64; 784]>, Digit) = {
            // Теперь обучаем модель только по training_image_indices.
            let (training_image_count_by_digit, pixel_brightness_sums_by_digit): (
                DataByDigit<usize>,
                DataByDigit<[f64; 784]>,
            ) = {
                // Для каждой цифры считаем только обучающие примеры.
                // Эти числа нужны для деления суммы пикселей на число картинок и выбора самой частой цифры.
                let mut training_image_count_by_digit: DataByDigit<usize> = DataByDigit {
                    zero: 0usize,
                    one: 0usize,
                    two: 0usize,
                    three: 0usize,
                    four: 0usize,
                    five: 0usize,
                    six: 0usize,
                    seven: 0usize,
                    eight: 0usize,
                    nine: 0usize,
                };
                // Например, pixel_brightness_sums_by_digit.seven[p] — сумма яркости пикселя p у семёрок train.
                // После деления получим десять средних изображений, каждое из 784 чисел.
                let mut pixel_brightness_sums_by_digit: DataByDigit<[f64; 784]> = DataByDigit {
                    zero: [0.0; 784],
                    one: [0.0; 784],
                    two: [0.0; 784],
                    three: [0.0; 784],
                    four: [0.0; 784],
                    five: [0.0; 784],
                    six: [0.0; 784],
                    seven: [0.0; 784],
                    eight: [0.0; 784],
                    nine: [0.0; 784],
                };
                // Типы переменных: image_index: usize.
                for &image_index in &training_image_indices {
                    let (normalized_pixels, actual_digit): &([f64; 784], Digit) =
                        &labeled_images[image_index];
                    // Выбираем счётчик и сумму яркостей для правильной цифры.
                    let (training_image_count, pixel_brightness_sums): (
                        &mut usize,
                        &mut [f64; 784],
                    ) = match *actual_digit {
                        Digit::Zero => (
                            &mut training_image_count_by_digit.zero,
                            &mut pixel_brightness_sums_by_digit.zero,
                        ),
                        Digit::One => (
                            &mut training_image_count_by_digit.one,
                            &mut pixel_brightness_sums_by_digit.one,
                        ),
                        Digit::Two => (
                            &mut training_image_count_by_digit.two,
                            &mut pixel_brightness_sums_by_digit.two,
                        ),
                        Digit::Three => (
                            &mut training_image_count_by_digit.three,
                            &mut pixel_brightness_sums_by_digit.three,
                        ),
                        Digit::Four => (
                            &mut training_image_count_by_digit.four,
                            &mut pixel_brightness_sums_by_digit.four,
                        ),
                        Digit::Five => (
                            &mut training_image_count_by_digit.five,
                            &mut pixel_brightness_sums_by_digit.five,
                        ),
                        Digit::Six => (
                            &mut training_image_count_by_digit.six,
                            &mut pixel_brightness_sums_by_digit.six,
                        ),
                        Digit::Seven => (
                            &mut training_image_count_by_digit.seven,
                            &mut pixel_brightness_sums_by_digit.seven,
                        ),
                        Digit::Eight => (
                            &mut training_image_count_by_digit.eight,
                            &mut pixel_brightness_sums_by_digit.eight,
                        ),
                        Digit::Nine => (
                            &mut training_image_count_by_digit.nine,
                            &mut pixel_brightness_sums_by_digit.nine,
                        ),
                    };
                    *training_image_count += 1;
                    // Типы переменных: pixel_brightness_sum: &mut f64, pixel_brightness: &f64.
                    for (pixel_brightness_sum, pixel_brightness) in
                        pixel_brightness_sums.iter_mut().zip(normalized_pixels)
                    {
                        // Складываем яркости в одной и той же координате изображения.
                        *pixel_brightness_sum += pixel_brightness;
                    }
                }
                if all_digits.into_iter().any(|digit: Digit| -> bool {
                    (match digit {
                        Digit::Zero => training_image_count_by_digit.zero,
                        Digit::One => training_image_count_by_digit.one,
                        Digit::Two => training_image_count_by_digit.two,
                        Digit::Three => training_image_count_by_digit.three,
                        Digit::Four => training_image_count_by_digit.four,
                        Digit::Five => training_image_count_by_digit.five,
                        Digit::Six => training_image_count_by_digit.six,
                        Digit::Seven => training_image_count_by_digit.seven,
                        Digit::Eight => training_image_count_by_digit.eight,
                        Digit::Nine => training_image_count_by_digit.nine,
                    }) == 0
                }) {
                    return Err("В train нужны все десять классов".into());
                }
                (
                    training_image_count_by_digit,
                    pixel_brightness_sums_by_digit,
                )
            };
            // Готовые средние передаём оценке без mut.
            let mean_image_by_digit: DataByDigit<[f64; 784]> = {
                // Для каждой цифры делим сумму яркостей на число её обучающих картинок.
                // Обучение этой простой модели на этом заканчивается — итераций и градиентов здесь нет.
                let mut mean_image_by_digit: DataByDigit<[f64; 784]> =
                    pixel_brightness_sums_by_digit;
                // Типы переменных: digit: Digit.
                for digit in all_digits {
                    let (mean_image, image_count): (&mut [f64; 784], usize) = match digit {
                        Digit::Zero => (
                            &mut mean_image_by_digit.zero,
                            training_image_count_by_digit.zero,
                        ),
                        Digit::One => (
                            &mut mean_image_by_digit.one,
                            training_image_count_by_digit.one,
                        ),
                        Digit::Two => (
                            &mut mean_image_by_digit.two,
                            training_image_count_by_digit.two,
                        ),
                        Digit::Three => (
                            &mut mean_image_by_digit.three,
                            training_image_count_by_digit.three,
                        ),
                        Digit::Four => (
                            &mut mean_image_by_digit.four,
                            training_image_count_by_digit.four,
                        ),
                        Digit::Five => (
                            &mut mean_image_by_digit.five,
                            training_image_count_by_digit.five,
                        ),
                        Digit::Six => (
                            &mut mean_image_by_digit.six,
                            training_image_count_by_digit.six,
                        ),
                        Digit::Seven => (
                            &mut mean_image_by_digit.seven,
                            training_image_count_by_digit.seven,
                        ),
                        Digit::Eight => (
                            &mut mean_image_by_digit.eight,
                            training_image_count_by_digit.eight,
                        ),
                        Digit::Nine => (
                            &mut mean_image_by_digit.nine,
                            training_image_count_by_digit.nine,
                        ),
                    };
                    // Типы переменных: pixel_brightness: &mut f64.
                    for pixel_brightness in mean_image {
                        *pixel_brightness /= image_count as f64;
                    }
                }
                mean_image_by_digit
            };
            // Baseline — постоянный прогноз самой частой цифры train. Он показывает,
            // насколько модель лучше простого ответа без анализа пикселей.
            // При равной частоте выбираем меньшую цифру; метки test в выборе не участвуют.
            let most_common_training_digit: Digit = all_digits
                .into_iter()
                .max_by_key(|&digit: &Digit| -> (usize, std::cmp::Reverse<usize>) {
                    let digit_index: usize = digit as usize;
                    (
                        (match digit {
                            Digit::Zero => training_image_count_by_digit.zero,
                            Digit::One => training_image_count_by_digit.one,
                            Digit::Two => training_image_count_by_digit.two,
                            Digit::Three => training_image_count_by_digit.three,
                            Digit::Four => training_image_count_by_digit.four,
                            Digit::Five => training_image_count_by_digit.five,
                            Digit::Six => training_image_count_by_digit.six,
                            Digit::Seven => training_image_count_by_digit.seven,
                            Digit::Eight => training_image_count_by_digit.eight,
                            Digit::Nine => training_image_count_by_digit.nine,
                        }),
                        std::cmp::Reverse(digit_index),
                    )
                })
                .unwrap();
            println!(
                "train: n={}, baseline digit={}; split: every fifth per class",
                all_digits
                    .into_iter()
                    .map(|digit: Digit| -> usize {
                        match digit {
                            Digit::Zero => training_image_count_by_digit.zero,
                            Digit::One => training_image_count_by_digit.one,
                            Digit::Two => training_image_count_by_digit.two,
                            Digit::Three => training_image_count_by_digit.three,
                            Digit::Four => training_image_count_by_digit.four,
                            Digit::Five => training_image_count_by_digit.five,
                            Digit::Six => training_image_count_by_digit.six,
                            Digit::Seven => training_image_count_by_digit.seven,
                            Digit::Eight => training_image_count_by_digit.eight,
                            Digit::Nine => training_image_count_by_digit.nine,
                        }
                    })
                    .sum::<usize>(),
                most_common_training_digit as usize
            );

            (mean_image_by_digit, most_common_training_digit)
        };
        // Общие evaluate_digit_predictions/print_prediction_report проверяют отложенные индексы, не изменяя обученные mean_image_by_digit.
        print_prediction_report(
            "validation",
            evaluate_digit_predictions(
                &mean_image_by_digit,
                &labeled_images,
                &validation_image_indices,
                most_common_training_digit,
            ),
        );
        (mean_image_by_digit, most_common_training_digit)
    };
    // Test загружается после обучения; его данные и метрики остаются здесь.
    {
        let test_images: Vec<([f64; 784], Digit)> =
            load_labeled_png_images(&dataset_directory.join("test"))?;
        let image_indices: Vec<usize> = (0..test_images.len()).collect();
        print_prediction_report(
            "test",
            evaluate_digit_predictions(
                &mean_image_by_digit,
                &test_images,
                &image_indices,
                most_common_training_digit,
            ),
        );
    }

    Ok(())
}
