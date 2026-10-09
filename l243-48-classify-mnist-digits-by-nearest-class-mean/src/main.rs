fn main() -> Result<(), String> {
    // Урок 243. Путь данных: PNG -> 784 числа -> средние изображения -> прогноз цифры.
    // Каждая картинка имеет размер 28×28, поэтому в ней 28*28 = 784 пикселя.
    // У одной записи два поля: массив яркостей и правильная метка (цифра 0–9).
    // «Обучение» здесь — накопление и усреднение примеров каждой цифры.
    // Затем новую картинку сравниваем с десятью средними и выбираем ближайшее.
    //
    // Сначала подключён общий загрузчик PNG и объявлены типы урока.
    // Затем обучаем средние изображения и объявляем замыкание, которое использует готовую модель.
    // После этого выполняем проверку validation и test.
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

    // Digit и порядок цифр общие для всех четырёх уроков MNIST.
    use mnist_data::{ALL_DIGITS, Digit, load_labeled_png_images};
    // Результат распознавания одной картинки: правильная цифра и ответ модели.
    struct DigitPrediction {
        actual_digit: Digit,
        predicted_digit: Digit,
    }

    // У каждой цифры своё именованное поле. T задаёт содержимое поля:
    // usize для счётчика, [f64; 784] для суммы яркостей или среднего изображения.
    #[derive(Debug)]
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
    let all_digits: [Digit; 10] = ALL_DIGITS;

    // Корень PNG фиксирован относительно папки урока; аргументы запуска не требуются.
    let dataset_directory: std::path::PathBuf =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");

    // PNG читает общий загрузчик из mnist_data: сортировка, проверка формата и нормализация.

    // Сначала готовим три набора: обучение, проверку и итоговый тест.
    let (train_images, validation_images): (Vec<([f64; 784], Digit)>, Vec<([f64; 784], Digit)>) = {
        let original_train_images: Vec<([f64; 784], Digit)> =
            load_labeled_png_images(&dataset_directory.join("train"))?;
        let mut train_images: Vec<([f64; 784], Digit)> = Vec::new();
        let mut validation_images: Vec<([f64; 784], Digit)> = Vec::new();
        // Типы переменных: digit: Digit.
        for digit in all_digits {
            // Фильтруем картинки одной цифры, сохраняя порядок имён файлов.
            let images_of_current_digit: Vec<&([f64; 784], Digit)> = original_train_images
                .iter()
                .filter(|image: &&([f64; 784], Digit)| -> bool { image.1 == digit })
                .collect();
            // В train копируем все картинки, кроме каждой пятой внутри этой цифры.
            train_images.extend(
                images_of_current_digit
                    .iter()
                    .enumerate()
                    .filter(|(position, _): &(usize, &&([f64; 784], Digit))| -> bool {
                        position % 5 != 0
                    })
                    .map(
                        |(_, &image): (usize, &&([f64; 784], Digit))| -> ([f64; 784], Digit) {
                            *image
                        },
                    ),
            );
            // В validation копируем каждую пятую: позиции 0, 5, 10…
            validation_images.extend(
                images_of_current_digit
                    .iter()
                    .enumerate()
                    .filter(|(position, _): &(usize, &&([f64; 784], Digit))| -> bool {
                        position % 5 == 0
                    })
                    .map(
                        |(_, &image): (usize, &&([f64; 784], Digit))| -> ([f64; 784], Digit) {
                            *image
                        },
                    ),
            );
        }
        (train_images, validation_images)
    };
    // Официальный test уже хранится отдельно: его не берём из обучающих картинок.
    let test_images: Vec<([f64; 784], Digit)> =
        load_labeled_png_images(&dataset_directory.join("test"))?;
    let mean_image_by_digit: DataByDigit<[f64; 784]> = {
        // Теперь обучаем модель только по train_images.
        let (training_image_count_by_digit, zero_initialized_pixel_brightness_sums_by_digit): (
            DataByDigit<usize>,
            DataByDigit<[f64; 784]>,
        ) = {
            // Для каждой цифры считаем только обучающие примеры.
            // Эти числа нужны для деления суммы пикселей на число картинок.
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
            // Например, zero_initialized_pixel_brightness_sums_by_digit.seven[p] — сумма яркости пикселя p у семёрок train.
            // После деления получим десять средних изображений, каждое из 784 чисел.
            // Изначально суммы равны нулю: каждое из десяти полей содержит 784 значения 0.0.
            // Далее к ним прибавляем яркости пикселей обучающих картинок.
            let mut zero_initialized_pixel_brightness_sums_by_digit: DataByDigit<[f64; 784]> =
                DataByDigit {
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
            // Типы переменных: normalized_pixels: &[f64; 784], actual_digit: &Digit.
            for (normalized_pixels, actual_digit) in &train_images {
                // Выбираем счётчик и сумму яркостей для правильной цифры.
                let (
                    training_image_count_for_actual_digit,
                    zero_initialized_pixel_brightness_sums_for_actual_digit,
                ): (&mut usize, &mut [f64; 784]) = match *actual_digit {
                    Digit::Zero => (
                        &mut training_image_count_by_digit.zero,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.zero,
                    ),
                    Digit::One => (
                        &mut training_image_count_by_digit.one,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.one,
                    ),
                    Digit::Two => (
                        &mut training_image_count_by_digit.two,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.two,
                    ),
                    Digit::Three => (
                        &mut training_image_count_by_digit.three,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.three,
                    ),
                    Digit::Four => (
                        &mut training_image_count_by_digit.four,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.four,
                    ),
                    Digit::Five => (
                        &mut training_image_count_by_digit.five,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.five,
                    ),
                    Digit::Six => (
                        &mut training_image_count_by_digit.six,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.six,
                    ),
                    Digit::Seven => (
                        &mut training_image_count_by_digit.seven,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.seven,
                    ),
                    Digit::Eight => (
                        &mut training_image_count_by_digit.eight,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.eight,
                    ),
                    Digit::Nine => (
                        &mut training_image_count_by_digit.nine,
                        &mut zero_initialized_pixel_brightness_sums_by_digit.nine,
                    ),
                };
                *training_image_count_for_actual_digit += 1;
                // println!("@{training_image_count_for_actual_digit}");
                // Типы переменных: pixel_brightness_sum: &mut f64, pixel_brightness: &f64.
                for (pixel_brightness_sum, pixel_brightness) in
                    zero_initialized_pixel_brightness_sums_for_actual_digit
                        .iter_mut()
                        .zip(normalized_pixels)
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
                zero_initialized_pixel_brightness_sums_by_digit,
            )
        };
        // println!("{training_image_count_by_digit:#?}");
        // println!("{zero_initialized_pixel_brightness_sums_by_digit:#?}");
        // Готовые средние передаём оценке без mut.
        let mean_image_by_digit: DataByDigit<[f64; 784]> = {
            // Для каждой цифры делим сумму яркостей на число её обучающих картинок.
            // Обучение этой простой модели на этом заканчивается — итераций и градиентов здесь нет.
            let mut mean_image_by_digit: DataByDigit<[f64; 784]> =
                zero_initialized_pixel_brightness_sums_by_digit;
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
        // println!("{mean_image_by_digit:#?}");
        println!("{training_image_count_by_digit:#?}");
        println!(
            "train: n={}; split: every fifth per class",
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
                .sum::<usize>()
        );

        mean_image_by_digit
    };
    // Проверяем распознавание выбранных картинок, не меняя средние изображения.
    // mean_image_by_digit берём из окружающего блока: это уже обученные средние изображения.
    // images_with_actual_digits — картинки с правильными ответами.
    // Для каждой картинки сохраняем правильную и предсказанную цифры, затем печатаем точность.
    let evaluate_digit_predictions = |images_with_actual_digits: &[([f64; 784], Digit)]| -> () {
        // Список пополняется только во время обхода проверяемых картинок.
        let predictions: Vec<DigitPrediction> = {
            let mut predictions: Vec<DigitPrediction> =
                Vec::with_capacity(images_with_actual_digits.len());
            // Типы переменных: normalized_pixels: &[f64; 784], actual_digit: &Digit.
            for (normalized_pixels, actual_digit) in images_with_actual_digits {
                // Проверяем все десять классов и выбираем наименьшее расстояние до среднего.
                // Метка текущей картинки не участвует в выборе; её используем позже для проверки ответа.
                let predicted_digit: Digit = all_digits
                    .into_iter()
                    .min_by(
                        |&first_candidate_digit: &Digit,
                         &second_candidate_digit: &Digit|
                         -> std::cmp::Ordering {
                            // Представляем картинку как точку с 784 координатами:
                            // каждая координата — нормализованная яркость одного пикселя, а не его положение (x, y).
                            // Среднее изображение цифры — тоже точка с 784 координатами.
                            // Сравниваем яркости в одинаковых позициях: чем меньше сумма квадратов разниц,
                            // тем ближе эти точки и тем больше картинка похожа на среднее изображение.
                            // min_by выбирает цифру, среднее изображение которой ближе всего.
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
                                            // println!(
                                            //     "digit={digit:?}, image_pixel_brightness={image_pixel_brightness}, mean_pixel_brightness={mean_pixel_brightness}"
                                            // );
                                            (image_pixel_brightness - mean_pixel_brightness).powi(2)
                                        },
                                    )
                                    .sum::<f64>()
                            };
                            let first = squared_distance_to_mean_image(first_candidate_digit);
                            let second = squared_distance_to_mean_image(second_candidate_digit);
                            println!("first {first}, second {second}");
                            first.total_cmp(&second)
                        },
                    )
                    .unwrap();
                predictions.push(DigitPrediction {
                    actual_digit: *actual_digit,
                    predicted_digit,
                });
            }
            predictions
        };
        let image_count: usize = predictions.len();
        // Ответ правильный, если предсказанная цифра совпадает с настоящей.
        let model_correct_count: usize = predictions
            .iter()
            .filter(|prediction: &&DigitPrediction| -> bool {
                prediction.actual_digit == prediction.predicted_digit
            })
            .count();
        // Accuracy = model_correct_count/image_count. Например, 8 из 10 дают 0.8.
        println!(
            "n={image_count}, accuracy={:.4}",
            model_correct_count as f64 / image_count as f64
        );
        // if dataset_name == "test" {
        //     println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
        //     // Типы переменных: actual_digit: Digit.
        //     for actual_digit in all_digits {
        //         // Для каждой предсказанной цифры считаем записи с нужной парой цифр.
        //         let counts_for_actual_digit: Vec<usize> = all_digits
        //             .into_iter()
        //             .map(|predicted_digit: Digit| -> usize {
        //                 predictions
        //                     .iter()
        //                     .filter(|prediction: &&DigitPrediction| -> bool {
        //                         prediction.actual_digit == actual_digit
        //                             && prediction.predicted_digit == predicted_digit
        //                     })
        //                     .count()
        //             })
        //             .collect();
        //         println!("{counts_for_actual_digit:?}");
        //     }
        // }
    };

    // Проверяем готовые наборы validation и test, не изменяя обученные средние изображения.
    evaluate_digit_predictions(&validation_images);
    evaluate_digit_predictions(&test_images);

    Ok(())
}
