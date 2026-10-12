fn main() -> Result<(), String> {
    use mnist_data::numeric_types::{
        BiasFiniteF32, CorrectPredictionFractionFiniteFrom0To1InclusiveF32, DigitScoreFiniteF32, GradientFiniteF32, NonNegativeFiniteF32,
        NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64, PixelBrightnessForComputationFiniteFrom0To1InclusiveF32, PixelWeightFiniteF32, ProbabilityFiniteFrom0To1InclusiveF32,
        ScoreGradientFiniteFromMinus1To1InclusiveF32,
    };
    // Урок 244. Учим модель узнавать цифру по 784 яркостям картинки 28×28.
    // Читай шаги сверху вниз: данные → устройство модели → операции над группой
    // картинок → подготовка обучения → эпохи → итоговая проверка.
    // В шагах 3–6 мы объявляем замыкания, а не запускаем обучение.
    // Их тела выполняются при вызовах в шагах 8–11. Общие операции нужны
    // и обучению, и проверке, поэтому объявлены перед обоими этапами.
    // Обучение меняет только 784×10 весов и 10 смещений, всего 7850 чисел.

    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя; Digit — метка цифры;
    // u64 — состояние генератора; String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 используется при чтении PNG, f32 — в вычислениях нейросети.
    // Яркости: PixelBrightnessForComputationFiniteFrom0To1InclusiveF32 (0..=1), веса: PixelWeightFiniteF32 (любой знак).
    // Градиенты оценок: ScoreGradientFiniteFromMinus1To1InclusiveF32 — конечные числа от -1 до 1.
    // Смещения: BiasFiniteF32, оценки: DigitScoreFiniteF32, производные: GradientFiniteF32 — конечные числа любого знака.
    // ProbabilityFiniteFrom0To1InclusiveF32 и CorrectPredictionFractionFiniteFrom0To1InclusiveF32 ограничены 0..=1; NonNegativeFiniteF32 допускает конечные числа >= 0.
    // Реализации проверок и закрытые поля находятся в модуле mnist_data::numeric_types.
    // Vec оставляем для переменного числа картинок, меток и предсказаний.
    // Массив [PixelWeightAndBias; 10] хранит параметры десяти цифр; PixelWeightAndBias — веса пикселей и смещение одной цифры.

    // Digit и порядок цифр общие для всех четырёх уроков MNIST.
    // Порядок соответствует папкам 0..9 и индексам классов в массивах.
    use mnist_data::{ALL_DIGITS, Digit, load_mnist_images, mnist_dataset_directory};

    // Шаг 1. Загружаем картинки и правильные ответы.
    // Train меняет веса; validation выбирает лучшую эпоху; test проверяет результат.
    // Каждая запись — 784 яркости от 0 до 1 и правильная цифра из имени папки.
    // Засекаем время загрузки данных, обучения и итоговой оценки.
    let started_at: std::time::Instant = std::time::Instant::now();
    // Общий путь к PNG задаётся в mnist_data; аргументы запуска не требуются.
    let dataset_directory: std::path::PathBuf = mnist_dataset_directory();
    // Размер группы общий для обучения и оценки.
    let batch_size: usize = 64;

    // PNG читает общий загрузчик из mnist_data: сортировка, проверка формата и нормализация.

    // Сначала готовим три набора: обучение, проверку и итоговый тест.
    let (
        train_images_with_actual_digits,
        validation_images_with_actual_digits,
        test_images_with_actual_digits,
    ): (
        Vec<([NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784], Digit)>,
        Vec<([NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784], Digit)>,
        Vec<([NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784], Digit)>,
    ) = load_mnist_images(&dataset_directory)?;

    // Шаг 2. Выбираем способ классификации и из него выводим структуру модели.
    // Сама задача «узнать цифру на картинке» не задаёт эти структуры.
    // В уроке 243 мы хранили средние изображения цифр. Здесь выбираем другой
    // способ решения — линейную модель, параметры которой будем обучать.
    //
    // 2.1. Вход — 784 яркости картинки 28×28. Индекс пикселя всегда обозначает
    // одно и то же место картинки. Выход должен быть одной из цифр 0..9.
    // Решаем вычислять десять оценок: «это ноль», «это единица», …, «это девятка».
    // Предсказанием будет цифра с наибольшей оценкой.
    //
    // 2.2. Теперь выбираем правило вычисления каждой оценки. Например, для семёрки:
    // оценка_7 = пиксель[0] × вес_7[0] + … + пиксель[783] × вес_7[783] + смещение_7.
    // Поэтому одной цифре нужны 784 веса — по одному на каждую позицию пикселя.
    // Положительный вес: увеличение яркости в этом месте повышает оценку цифры.
    // Отрицательный вес: увеличение яркости понижает оценку; нулевой не влияет.
    // Сам вес — коэффициент, а вклад пикселя — произведение его яркости на вес.
    // Например, яркость 0.8 при весе 0.5 добавляет к оценке 0.4.
    // Веса не меняют исходную картинку: они определяют, как использовать её яркости.
    //
    // 2.3. Смещение позволяет сдвигать оценку цифры независимо от пикселей.
    // Без него при всех нулевых яркостях оценка обязательно равнялась бы нулю.
    // Смещение относится ко всей оценке цифры, а не к отдельному пикселю.
    // Значения весов и смещения подбирает обучение по картинкам с известными ответами.
    #[derive(Clone)]
    struct PixelWeightAndBias {
        // 784 коэффициента для оценки ОДНОЙ цифры: индекс выбирает пиксель.
        pixel_weights: [PixelWeightFiniteF32; 784],
        // Одно смещение этой цифры; добавляется после суммы вкладов всех пикселей.
        bias: BiasFiniteF32,
    }
    // Производная может иметь любой знак, но не является весом или смещением.
    #[derive(Clone)]
    struct PixelWeightAndBiasGradients {
        pixel_weights: [GradientFiniteF32; 784],
        bias: GradientFiniteF32,
    }
    // Adam хранит средний градиент со знаком и неотрицательное среднее квадратов.
    #[derive(Clone, Copy)]
    struct AdamMoments {
        mean_gradient: GradientFiniteF32,
        mean_squared_gradient: NonNegativeFiniteF32,
    }
    #[derive(Clone)]
    struct PixelWeightAndBiasAdamMoments {
        pixel_weights: [AdamMoments; 784],
        bias: AdamMoments,
    }
    // 2.4. Каждая из десяти цифр получает собственный набор параметров.
    // Один пиксель может повышать оценку семёрки и понижать оценку восьмёрки:
    // для этих цифр у него разные веса. Всего обучаем 10 × (784 + 1) = 7850 чисел.
    // Модель — массив [PixelWeightAndBias; 10]; индекс выбирает цифру.
    // pixel_weights_and_biases_by_digit[7] — веса пикселей и смещение для оценки семёрки.
    // 2.5. Объединяем ОДНУ картинку и десять вычисленных для неё оценок.
    // Группа результатов — Vec<ImagePixelsWithActualDigitAndScoresByDigit>; каждая запись связывает пиксели
    // с их оценками. Пиксели нужны для градиентов весов, оценки — для ошибки и ответа.
    struct ImagePixelsWithActualDigitAndScoresByDigit {
        // Правильная цифра хранится с той же картинкой и её оценками.
        actual_digit: Digit,
        // 784 яркости одной картинки; индекс выбирает пиксель.
        pixels: [PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784],
        // Десять оценок этой картинки; индекс выбирает цифру. Это ещё не вероятности.
        scores_by_digit: [DigitScoreFiniteF32; 10],
    }
    // Шаг 3. Подготавливаем яркости выбранных картинок для вычислений.
    // Вход: ссылки на пары с f64. Выход: пары с проверенной PixelBrightnessForComputationFiniteFrom0To1InclusiveF32.
    // Деление на группы выполняет chunks(batch_size); здесь только преобразование яркостей.
    // Явно задаём правила преобразования яркости из f64 в f32.
    // Стандартного f32::try_from(f64) нет; для собственного типа можно реализовать TryFrom.
    // Конечность и диапазон проверены при загрузке. Здесь не требуем точного равенства:
    // большинство дробей pixel_byte / 255 нельзя точно представить в f32.
    // Округление разрешено: все 256 исходных уровней PNG остаются различимыми.
    // Например, 128 / 255 ≈ 0.5019607843 превращается в f32 ≈ 0.5019608.
    // Входной тип гарантирует допустимую яркость; преобразование НЕ гарантирует отсутствие округления.
    // Яркость, подготовленная для вычислений модели: f32 занимает 4 байта вместо 8 у f64.
    // Это выбор представления для уменьшения памяти числовых данных; ускорение зависит
    // от оборудования и реализации. Проверки ниже относятся именно к яркости 0..=1,
    // поэтому тип не является универсальным преобразователем любых чисел f64.
    let convert_image_pixel_brightness_from_f64_to_f32 = |images_with_actual_digits: &[&(
        [NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784],
        Digit,
    )]|
     -> Result<
        Vec<([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)>,
        String,
    > {
        let mut converted_images: Vec<([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)> =
            Vec::with_capacity(images_with_actual_digits.len());
        // Типы переменных: pixels: &[NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784], actual_digit: &Digit.
        for &(pixels, actual_digit) in images_with_actual_digits {
            // Загрузчик вернул f64; вычисления модели используют f32.
            let mut converted_pixels: [PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784] =
                [PixelBrightnessForComputationFiniteFrom0To1InclusiveF32::ZERO; 784];
            // Типы переменных: pixel_index: usize.
            for pixel_index in 0..pixels.len() {
                converted_pixels[pixel_index] = PixelBrightnessForComputationFiniteFrom0To1InclusiveF32::try_from(
                    pixels[pixel_index],
                )
                .map_err(|error: String| -> String {
                    format!("Пиксель {pixel_index}: {error}")
                })?;
            }
            converted_images.push((converted_pixels, *actual_digit));
        }
        Ok(converted_images)
    };

    // Яркости не меняются при обучении: преобразуем каждый набор ровно один раз.
    // Дальше группы содержат ссылки на готовые пары с типизированной яркостью; меняется лишь порядок ссылок.
    let train_images_with_actual_digits: Vec<([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)> = {
        let images_with_actual_digits_references: Vec<&([NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784], Digit)> =
            train_images_with_actual_digits.iter().collect();
        convert_image_pixel_brightness_from_f64_to_f32(&images_with_actual_digits_references)?
    };
    let validation_images_with_actual_digits: Vec<(
        [PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784],
        Digit,
    )> = {
        let images_with_actual_digits_references: Vec<&([NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784], Digit)> =
            validation_images_with_actual_digits.iter().collect();
        convert_image_pixel_brightness_from_f64_to_f32(&images_with_actual_digits_references)?
    };
    let test_images_with_actual_digits: Vec<([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)> = {
        let images_with_actual_digits_references: Vec<&([NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64; 784], Digit)> =
            test_images_with_actual_digits.iter().collect();
        convert_image_pixel_brightness_from_f64_to_f32(&images_with_actual_digits_references)?
    };

    // Шаг 4. Описываем прямой проход: пиксели → десять оценок каждой картинки.
    // Проверенная яркость умножается на вес со знаком; результат может быть отрицательным.
    // Forward — прямой проход: из пикселей получаем оценки десяти цифр.
    // Сохраняем входы и оценки: они нужны для расчёта градиентов.
    let build_images_with_digit_scores =
        |pixel_weights_and_biases_by_digit: &[PixelWeightAndBias; 10],
         converted_images: &[&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)]|
         -> Result<Vec<ImagePixelsWithActualDigitAndScoresByDigit>, String> {
            let mut images_with_scores: Vec<ImagePixelsWithActualDigitAndScoresByDigit> =
                Vec::with_capacity(converted_images.len());
            // Типы переменных: pixels: &[PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784].
            for &(pixels, actual_digit) in converted_images {
                let mut image: ImagePixelsWithActualDigitAndScoresByDigit = ImagePixelsWithActualDigitAndScoresByDigit {
                    actual_digit: *actual_digit,
                    pixels: *pixels,
                    scores_by_digit: [DigitScoreFiniteF32::ZERO; 10],
                };
                // Оценка каждой цифры = сумма(яркость × её вес пикселя) + её смещение.
                // Типы переменных: digit_index: usize.
                for digit_index in 0..pixel_weights_and_biases_by_digit.len() {
                    let pixel_weights_and_bias: &PixelWeightAndBias =
                        &pixel_weights_and_biases_by_digit[digit_index];
                    // Сумма — промежуточное f32; проверяем конечность готовой оценки.
                    let mut score_by_digit: f32 = 0.0;
                    // Типы переменных: pixel_index: usize.
                    for pixel_index in 0..pixel_weights_and_bias.pixel_weights.len() {
                        score_by_digit += image.pixels[pixel_index].value()
                            * pixel_weights_and_bias.pixel_weights[pixel_index].value();
                    }
                    score_by_digit += pixel_weights_and_bias.bias.value();
                    image.scores_by_digit[digit_index] = DigitScoreFiniteF32::try_from(score_by_digit)?;
                }
                images_with_scores.push(image);
            }
            Ok(images_with_scores)
        };
    // Шаг 5. Описываем расчёт вероятностей, ошибки и производных оценок.
    // Это последовательность: оценки → softmax → loss → градиенты.
    // Cross-entropy — среднее отрицательных натуральных логарифмов вероятностей правильных цифр.
    // Если правильному классу дана большая вероятность, ошибка мала; если малая — велика.
    // Вероятность правильной цифры 0.9 даёт ошибку около 0.105, а 0.1 — около 2.303.
    // Loss не равна доле неверных ответов: учитывает уверенность даже при правильном прогнозе.
    // Вычисляем среднюю ошибку и её производные по оценкам цифр для обновления весов.
    // У каждой картинки берём scores_by_digit; пиксели здесь не изменяем.
    // Вычислить результат возведения 2.71828… в переданную степень (power).
    // Это экспонента (exp). В названии 2_71828 — приближение основания:
    // вычисление exp использует число e, а не усечённое значение 2.71828.
    // Число Эйлера e ≈ 2.718281828459045; его десятичная запись бесконечна.
    // exp(power) = e^power. Здесь ^ обозначает степень в формуле, не оператор Rust.
    // Пример: exp(2) = e² ≈ 2.718281828459045 × 2.718281828459045 ≈ 7.389.
    // Ещё примеры: exp(0) = 1; exp(-1) = 1/e ≈ 0.368.
    // В softmax так преобразуем оценки перед делением на сумму.
    // В математике результат всегда положительный; в f32 он может округлиться
    // до 0 или переполниться до бесконечности. Проверка результата — в месте вызова.
    fn calculate_result_of_raising_2_71828_to_power(power: f32) -> f32 {
        power.exp()
    }

    // Вычислить степень, в которую нужно возвести 2.71828…,
    // чтобы получить переданное число (number).
    // Это натуральный логарифм (ln), основание — то же число e ≈ 2.718281828459045.
    // Находим степень, в которую надо возвести это число e, чтобы получить number.
    // ln(number) отвечает: «e в какой степени даст number?»
    // Пример: ln(7.389...) ≈ 2, потому что e² ≈ 7.389.
    // Ещё примеры: ln(1) = 0; ln(0.368...) ≈ -1.
    // exp и ln — обратные операции: 2 → exp → 7.389... → ln → 2.
    // В f32 результат этой цепочки может отличаться из-за округления.
    // При расчёте ошибки используем -ln(вероятность правильной цифры):
    // -ln(0.9) ≈ 0.105, а -ln(0.1) ≈ 2.303 — меньшая вероятность даёт большую ошибку.
    // Конечный результат требует положительного конечного number.
    // ln(0) даёт -∞; ln отрицательного числа даёт NaN («не число»).
    fn calculate_power_of_2_71828_to_get_number(number: f32) -> f32 {
        number.ln()
    }

    struct MeanLossAndScoreGradientsByDigit {
        // Средняя ошибка группы: меньше — выше вероятность правильных цифр.
        mean_loss: f32,
        // [картинка][цифра]: производная средней ошибки по оценке этой цифры.
        // Это градиенты оценок; градиенты весов и смещений вычисляются дальше.
        scores_gradients_by_digit: Vec<[ScoreGradientFiniteFromMinus1To1InclusiveF32; 10]>,
    }
    let calculate_mean_loss_and_score_gradients_by_digit =
        |images_with_scores: &[ImagePixelsWithActualDigitAndScoresByDigit]| -> Result<MeanLossAndScoreGradientsByDigit, String> {
            if images_with_scores.is_empty() {
                return Err("Нельзя вычислить среднюю ошибку пустой группы".into());
            }
            // Для каждой картинки отдельно вычисляем вероятности, ошибку и градиенты.
            let (scores_gradients_by_digit, total_loss): (Vec<[ScoreGradientFiniteFromMinus1To1InclusiveF32; 10]>, f32) = {
                let mut scores_gradients_by_digit: Vec<[ScoreGradientFiniteFromMinus1To1InclusiveF32; 10]> =
                    Vec::with_capacity(images_with_scores.len());
                let mut total_loss: f32 = 0.0;
                // Типы переменных: image_with_score: &ImagePixelsWithActualDigitAndScoresByDigit.
                for image_with_score in images_with_scores {
                    // Digit имеет только варианты 0..9; оценки всегда содержат 10 элементов.
                    // Перед exp вычитаем максимум: вероятности сохраняются, переполнения нет.
                    let maximum_score_by_digit: f32 = image_with_score
                        .scores_by_digit
                        .iter()
                        .map(|score_by_digit: &DigitScoreFiniteF32| -> f32 { score_by_digit.value() })
                        .fold(f32::NEG_INFINITY, f32::max);
                    // Например, оценка 3 при максимуме 5 даёт exp(3 - 5) = e⁻² ≈ 0.135.
                    // Максимальная оценка 5 даёт exp(5 - 5) = e⁰ = 1.
                    // Это ещё не вероятности: ниже делим каждое число на сумму всех десяти.
                    // Каждый результат exp проверяется: NaN и бесконечность вернут Err.
                    // Отдельный блок ограничивает изменение массива этапом его заполнения.
                    let exponentials_by_digit: [NonNegativeFiniteF32; 10] = {
                        let mut exponentials_by_digit: [NonNegativeFiniteF32; 10] =
                            [NonNegativeFiniteF32::try_from(0.0)?; 10];
                        // Типы переменных: digit_index: usize.
                        for digit_index in 0..image_with_score.scores_by_digit.len() {
                            let score_by_digit: f32 =
                                image_with_score.scores_by_digit[digit_index].value();
                            exponentials_by_digit[digit_index] = NonNegativeFiniteF32::try_from(
                                calculate_result_of_raising_2_71828_to_power(
                                    score_by_digit - maximum_score_by_digit,
                                ),
                            )?;
                        }
                        exponentials_by_digit
                    };
                    let exponential_sum: f32 = exponentials_by_digit
                        .iter()
                        .map(|exponential: &NonNegativeFiniteF32| -> f32 { exponential.value() })
                        .sum();
                    // Softmax: делим экспоненты на сумму и получаем вероятности десяти цифр.
                    // При конечных оценках хотя бы одна экспонента равна exp(0) = 1.
                    let probabilities_by_digit: [ProbabilityFiniteFrom0To1InclusiveF32; 10] = {
                        let mut probabilities_by_digit: [ProbabilityFiniteFrom0To1InclusiveF32; 10] =
                            [ProbabilityFiniteFrom0To1InclusiveF32::ZERO; 10];
                        // Типы переменных: probabilities_by_digit_index: usize.
                        for probabilities_by_digit_index in 0..probabilities_by_digit.len() {
                            probabilities_by_digit[probabilities_by_digit_index] =
                                ProbabilityFiniteFrom0To1InclusiveF32::try_from(
                                    exponentials_by_digit[probabilities_by_digit_index].value()
                                        / exponential_sum,
                                )?;
                        }
                        probabilities_by_digit
                    };

                    // Ошибка = -ln(вероятность правильной цифры).
                    // Вероятность правильной цифры = exp(её оценка - максимум) / сумма экспонент.
                    // Поэтому -ln(вероятность) = максимум + ln(сумма экспонент) - её оценка.
                    // Например: максимум = 5, сумма = 2, оценка правильной цифры = 4.
                    // Ошибка = 5 + ln(2) - 4 ≈ 5 + 0.693 - 4 = 1.693.
                    // Эта формула не берёт ln вероятности, которая могла округлиться до нуля.
                    let actual_digit_score: f32 = image_with_score.scores_by_digit
                        [image_with_score.actual_digit as usize]
                        .value();
                    total_loss += maximum_score_by_digit
                        + calculate_power_of_2_71828_to_get_number(exponential_sum)
                        - actual_digit_score;

                    // Производная оценки = вероятность минус 1 для правильной цифры,
                    // минус 0 для остальных. Делим на фактический размер группы,
                    // поскольку обучаемся по средней ошибке, включая неполный последний batch.
                    let gradients_by_digit: [ScoreGradientFiniteFromMinus1To1InclusiveF32; 10] = {
                        let mut gradients_by_digit: [ScoreGradientFiniteFromMinus1To1InclusiveF32; 10] =
                            [ScoreGradientFiniteFromMinus1To1InclusiveF32::ZERO; 10];
                        // Типы переменных: gradients_by_digit_index: usize.
                        for gradients_by_digit_index in 0..gradients_by_digit.len() {
                            let target: f32 = if gradients_by_digit_index
                                == image_with_score.actual_digit as usize
                            {
                                1.0
                            } else {
                                0.0
                            };
                            gradients_by_digit[gradients_by_digit_index] =
                                ScoreGradientFiniteFromMinus1To1InclusiveF32::try_from(
                                    (probabilities_by_digit[gradients_by_digit_index].value()
                                        - target)
                                        / images_with_scores.len() as f32,
                                )?;
                        }
                        gradients_by_digit
                    };
                    scores_gradients_by_digit.push(gradients_by_digit);
                }
                (scores_gradients_by_digit, total_loss)
            };
            Ok(MeanLossAndScoreGradientsByDigit {
                mean_loss: total_loss / images_with_scores.len() as f32,
                scores_gradients_by_digit,
            })
        };
    // Шаг 6. Собираем проверку набора из операций шагов 3–5.
    struct DigitActualAndPredicted {
        actual_digit: Digit,
        predicted_digit: Digit,
    }

    // Хотя бы одна пара цифр. Создаём через TryFrom с проверкой длины.
    // Для чтения используем iter; методов изменения списка не предоставляем.
    struct NonEmptyDigitActualAndPredictedVec(Vec<DigitActualAndPredicted>);

    impl TryFrom<Vec<DigitActualAndPredicted>> for NonEmptyDigitActualAndPredictedVec {
        type Error = String;

        fn try_from(digits_actual_and_predicted: Vec<DigitActualAndPredicted>) -> Result<Self, Self::Error> {
            if digits_actual_and_predicted.is_empty() {
                return Err("Список правильных и предсказанных цифр должен быть непустым".into());
            }
            Ok(Self(digits_actual_and_predicted))
        }
    }

    impl NonEmptyDigitActualAndPredictedVec {
        fn iter(&self) -> std::slice::Iter<'_, DigitActualAndPredicted> {
            self.0.iter()
        }
    }

    struct PredictionEvaluation {
        mean_loss: f32,
        // Доля правильных ответов от 0 до 1: 0.92 соответствует 92%.
        correct_prediction_fraction: CorrectPredictionFractionFiniteFrom0To1InclusiveF32,
        digits_actual_and_predicted: NonEmptyDigitActualAndPredictedVec,
    }
    // Оценка читает текущую модель, не обновляя веса. Передаём целый готовый набор.
    // Модель остаётся параметром: validation проверяет разные веса после каждой эпохи.
    let measure_prediction_quality = |pixel_weights_and_biases_by_digit: &[PixelWeightAndBias;
                                           10],
                                      images_with_actual_digits: &[(
        [PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784],
        Digit,
    )]|
     -> Result<PredictionEvaluation, String> {
        if images_with_actual_digits.is_empty() {
            return Err("Нельзя оценить качество на пустом наборе".into());
        }
        if batch_size == 0 {
            return Err("Размер группы должен быть больше нуля".into());
        }
        let (digits_actual_and_predicted, total_loss): (Vec<DigitActualAndPredicted>, f32) = {
            // Ссылки выбирают существующие записи; каждая группа содержит картинки вместе с метками.
            let images_with_actual_digits_references: Vec<&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)> =
                images_with_actual_digits.iter().collect();
            // Наибольшая оценка выбирает цифру; при равенстве предпочитаем меньшую.
            let digit_with_highest_score = |digit_scores_for_image: &[DigitScoreFiniteF32; 10]| -> Digit {
                let predicted_digit_index: usize = (0..digit_scores_for_image.len())
                    .max_by(
                        |&first_digit_index: &usize,
                         &second_digit_index: &usize|
                         -> std::cmp::Ordering {
                            digit_scores_for_image[first_digit_index]
                                .value()
                                .total_cmp(&digit_scores_for_image[second_digit_index].value())
                                .then_with(|| -> std::cmp::Ordering {
                                    second_digit_index.cmp(&first_digit_index)
                                })
                        },
                    )
                    .unwrap();
                ALL_DIGITS[predicted_digit_index]
            };
            let mut digits_actual_and_predicted: Vec<DigitActualAndPredicted> =
                Vec::with_capacity(images_with_actual_digits.len());
            let mut total_loss: f32 = 0.0;
            // Типы переменных: batch_images_with_actual_digits_references: &[&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)].
            for batch_images_with_actual_digits_references in images_with_actual_digits_references.chunks(batch_size) {
                let images_with_scores: Vec<ImagePixelsWithActualDigitAndScoresByDigit> = build_images_with_digit_scores(
                    pixel_weights_and_biases_by_digit,
                    batch_images_with_actual_digits_references,
                )?;
                // loss одного batch — средняя: умножаем на его фактический размер.
                // Затем делим общую сумму на число картинок, учитывая неполный последний batch.
                total_loss += calculate_mean_loss_and_score_gradients_by_digit(&images_with_scores)?.mean_loss
                    * batch_images_with_actual_digits_references.len() as f32;
                // Типы переменных: image: &ImagePixelsWithActualDigitAndScoresByDigit.
                for image in &images_with_scores {
                    digits_actual_and_predicted.push(DigitActualAndPredicted {
                        actual_digit: image.actual_digit,
                        predicted_digit: digit_with_highest_score(&image.scores_by_digit),
                    });
                }
            }
            (digits_actual_and_predicted, total_loss)
        };
        let digits_actual_and_predicted = NonEmptyDigitActualAndPredictedVec::try_from(digits_actual_and_predicted)?;
        let correct_prediction_count: usize = digits_actual_and_predicted
            .iter()
            .filter(|prediction: &&DigitActualAndPredicted| -> bool {
                prediction.actual_digit == prediction.predicted_digit
            })
            .count();
        Ok(PredictionEvaluation {
            mean_loss: total_loss / images_with_actual_digits.len() as f32,
            correct_prediction_fraction: CorrectPredictionFractionFiniteFrom0To1InclusiveF32::try_from(
                correct_prediction_count as f32 / images_with_actual_digits.len() as f32,
            )?,
            digits_actual_and_predicted,
        })
    };
    // Шаг 7. Готовим обучение: порядок картинок, начальные веса и память Adam.
    // Из обучения выходят только выбранные веса и необходимые для отчёта значения.
    let (best_pixel_weights_and_biases_by_digit, best_epoch_number): (
        [PixelWeightAndBias; 10],
        usize,
    ) = {
        // Создаём нулевые параметры модели. Градиенты и память Adam имеют свои типы.
        // Каждый вызов создаёт независимый массив; затем модель получает случайные веса.
        let create_zero_pixel_weights_and_biases = || -> [PixelWeightAndBias; 10] {
            std::array::from_fn(|_: usize| -> PixelWeightAndBias {
                PixelWeightAndBias {
                    pixel_weights: [PixelWeightFiniteF32::ZERO; 784],
                    bias: BiasFiniteF32::ZERO,
                }
            })
        };
        // Настройки нужны только обучению; для экспериментов меняй их здесь.
        // epoch_count — число проходов по train, learning_rate — масштаб шага Adam.
        // random_seed задаёт воспроизводимую последовательность перемешиваний и начальных весов.
        let epoch_count: usize = 15;
        let learning_rate: f32 = 0.001;
        let random_seed: u64 = 42;
        println!(
            "Linear 784 -> 10: epochs={epoch_count}, batch={batch_size}, lr={learning_rate}, seed={random_seed}, data={}",
            dataset_directory.display()
        );
        // Общий SplitMix64 для перемешивания и инициализации весов.
        // Состояние передаём явно: оба потребителя продолжают одну последовательность seed.
        let next_random_number = |random_state: &mut u64| -> u64 {
            *random_state = random_state.wrapping_add(0x9e3779b97f4a7c15);
            let mut random_bits: u64 = *random_state;
            random_bits = (random_bits ^ (random_bits >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            random_bits = (random_bits ^ (random_bits >> 27)).wrapping_mul(0x94d049bb133111eb);
            random_bits ^ (random_bits >> 31)
        };

        // Fisher–Yates: идём с конца и меняем текущий индекс со случайным индексом 0..=current_position.
        // Меняется порядок записей train; сами картинки и метки не изменяются.
        let shuffle_training_images =
            |images_with_actual_digits_references: &mut [&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)],
             random_state: &mut u64|
             -> () {
                // Типы переменных: current_position: usize.
                for current_position in (1..images_with_actual_digits_references.len()).rev() {
                    let random_position: usize =
                        (next_random_number(random_state) % (current_position as u64 + 1)) as usize;
                    images_with_actual_digits_references.swap(current_position, random_position);
                }
            };
        // Перемешиваем ссылки на записи готового train. Порядок RNG совпадает с прежним уроком:
        // сначала одно перемешивание, затем инициализация весов, затем эпохи.
        let (training_image_order, random_state): (
            Vec<&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)>,
            u64,
        ) = {
            let mut training_image_order: Vec<&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)> =
                train_images_with_actual_digits.iter().collect();
            let mut random_state: u64 = random_seed;
            shuffle_training_images(&mut training_image_order, &mut random_state);
            (training_image_order, random_state)
        };
        // Проверяем все десять цифр, не используя validation или test.
        if ALL_DIGITS.into_iter().any(|digit: Digit| -> bool {
            !train_images_with_actual_digits.iter().any(
                |image: &([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)| -> bool {
                    image.1 == digit
                },
            )
        }) {
            return Err("Train должен содержать примеры всех десяти цифр".into());
        }
        println!(
            "train={}, validation={}; split=every fifth per class, sorted PNG filenames",
            train_images_with_actual_digits.len(),
            validation_images_with_actual_digits.len()
        );
        let (pixel_weights_and_biases_by_digit, random_state): ([PixelWeightAndBias; 10], u64) = {
            let mut random_state: u64 = random_state;
            // Создаём десять наборов параметров: веса пикселей и смещение каждой цифры.
            let mut pixel_weights_and_biases_by_digit: [PixelWeightAndBias; 10] =
                create_zero_pixel_weights_and_biases();
            let initial_weight_bound: f32 = (6.0 / 784.0_f32).sqrt();
            // Порядок случайных чисел сохраняем: для каждого пикселя перебираем цифры.
            // Храним полученные веса уже в параметрах соответствующей цифры.
            // Типы переменных: pixel_index: usize.
            for pixel_index in 0..784 {
                // Типы переменных: digit_index: usize.
                for digit_index in 0..pixel_weights_and_biases_by_digit.len() {
                    let random_fraction: f32 =
                        (next_random_number(&mut random_state) >> 40) as f32 / 16777216.0;
                    pixel_weights_and_biases_by_digit[digit_index].pixel_weights[pixel_index] =
                        PixelWeightFiniteF32::try_from(
                            (2.0 * random_fraction - 1.0) * initial_weight_bound,
                        )?;
                }
            }
            (pixel_weights_and_biases_by_digit, random_state)
        };

        let adam_moments_by_digit: [PixelWeightAndBiasAdamMoments; 10] =
            std::array::from_fn(|_: usize| -> PixelWeightAndBiasAdamMoments {
                let zero_moments: AdamMoments = AdamMoments {
                    mean_gradient: GradientFiniteF32::ZERO,
                    mean_squared_gradient: NonNegativeFiniteF32::ZERO,
                };
                PixelWeightAndBiasAdamMoments {
                    pixel_weights: [zero_moments; 784],
                    bias: zero_moments,
                }
            });
        // Шаг 8. Проверяем начальную модель на validation — это эпоха 0.
        // Оцениваем ещё не обученную сеть: это точка отсчёта для сравнения.
        let initial_validation_loss: f32 = {
            let initial_evaluation: PredictionEvaluation = measure_prediction_quality(
                &pixel_weights_and_biases_by_digit,
                &validation_images_with_actual_digits,
            )?;

            println!(
                "epoch=0 validation_loss={:.5} validation_accuracy={:.4}",
                initial_evaluation.mean_loss,
                initial_evaluation.correct_prediction_fraction.value()
            );
            initial_evaluation.mean_loss
        };
        // Шаг 9. Обучаем модель: повторяем проход по всем группам 15 раз.
        // Всё изменяемое состояние обучения живёт только в этом блоке эпох.
        // За его пределами доступны best_pixel_weights_and_biases_by_digit и best_epoch_number без mut.
        let (best_pixel_weights_and_biases_by_digit, best_epoch_number): (
            [PixelWeightAndBias; 10],
            usize,
        ) = {
            let mut pixel_weights_and_biases_by_digit: [PixelWeightAndBias; 10] =
                pixel_weights_and_biases_by_digit;
            let mut training_image_order: Vec<&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)> =
                training_image_order;
            let mut random_state: u64 = random_state;
            let mut adam_moments_by_digit: [PixelWeightAndBiasAdamMoments; 10] =
                adam_moments_by_digit;
            // Сохраняем отдельную копию лучших весов по validation loss.
            // Копия нужна, потому что следующие эпохи продолжат менять текущие pixel_weights_and_biases_by_digit.
            // В начале лучший кандидат — ещё не обученная сеть (эпоха 0).
            let mut best_pixel_weights_and_biases_by_digit: [PixelWeightAndBias; 10] =
                pixel_weights_and_biases_by_digit.clone();
            let mut best_validation_loss: f32 = initial_validation_loss;
            let mut best_epoch_number: usize = 0;
            // parameter_update_count — номер ОБНОВЛЕНИЯ весов, не эпохи. Он увеличивается после каждого batch
            // и нужен Adam для поправки на нулевые начальные средние.
            let mut parameter_update_count: i32 = 0i32;
            // Одна эпоха: пройти все batch train, затем оценить validation без обновления весов.
            // Test подготовлен заранее, но до выбора лучшей эпохи не участвует в вычислениях.
            // Типы переменных: epoch: usize.
            for epoch in 1..=epoch_count {
                // Меняем только порядок ссылок на обучающие записи. Генератор продолжает своё состояние,
                // поэтому каждая эпоха имеет новый, но воспроизводимый при том же seed порядок.
                shuffle_training_images(&mut training_image_order, &mut random_state);
                // Сумма меняется только при обходе train; для отчёта возвращаем готовую train loss.
                let total_training_loss: f32 = {
                    let mut total_training_loss: f32 = 0.0;
                    // Типы переменных: batch_images_with_actual_digits_references: &[&([PixelBrightnessForComputationFiniteFrom0To1InclusiveF32; 784], Digit)].
                    for batch_images_with_actual_digits_references in training_image_order.chunks(batch_size) {
                        // Сначала весь прямой и обратный проход по СТАРЫМ весам.
                        // Наружу из этого блока выходят только ошибка batch и градиенты параметров;
                        // входы, оценки цифр и промежуточные производные остаются внутри.
                        let (mean_loss, pixel_weight_and_bias_gradients_by_digit): (
                            f32,
                            [PixelWeightAndBiasGradients; 10],
                        ) = {
                            // 9.1. Текущая группа содержит ссылки на пиксели и правильные ответы (шаг 3).
                            // 9.2. По текущим весам получаем десять оценок на картинку (шаг 4).
                            let images_with_scores: Vec<ImagePixelsWithActualDigitAndScoresByDigit> =
                                build_images_with_digit_scores(
                                    &pixel_weights_and_biases_by_digit,
                                    batch_images_with_actual_digits_references,
                                )?;
                            // 9.3. Из оценок получаем ошибку и её производные (шаг 5).
                            // Получаем среднюю ошибку группы и производные по оценкам цифр — начало обратного прохода.
                            // Ошибка должна быть конечной; NaN/∞ означают, что численный расчёт нарушился.
                            let MeanLossAndScoreGradientsByDigit {
                                mean_loss,
                                scores_gradients_by_digit,
                            }: MeanLossAndScoreGradientsByDigit =
                                calculate_mean_loss_and_score_gradients_by_digit(&images_with_scores)?;
                            if !mean_loss.is_finite() {
                                return Err(
                                    "Ошибка обучения не конечна; уменьши learning-rate".into()
                                );
                            }
                            // 9.4. Переносим производные оценок на веса и смещения.
                            // Производные параметров нужны только для обновления после этого batch.
                            let pixel_weight_and_bias_gradients_by_digit: [PixelWeightAndBiasGradients; 10] = {
                                let mut gradients_by_digit: [PixelWeightAndBiasGradients; 10] =
                                    std::array::from_fn(|_: usize| -> PixelWeightAndBiasGradients {
                                        PixelWeightAndBiasGradients {
                                            pixel_weights: [GradientFiniteF32::ZERO; 784], bias: GradientFiniteF32::ZERO,
                                        }
                                    });
                                // Типы переменных: digit_index: usize.
                                for digit_index in 0..gradients_by_digit.len() {
                                    let gradients: &mut PixelWeightAndBiasGradients = &mut gradients_by_digit[digit_index];
                                    // Типы переменных: input_pixels_index: usize.
                                    for input_pixels_index in 0..images_with_scores.len() {
                                        let score_gradient: ScoreGradientFiniteFromMinus1To1InclusiveF32 = scores_gradients_by_digit[input_pixels_index][digit_index];
                                        // Типы переменных: pixel_index: usize.
                                        for pixel_index in 0..gradients.pixel_weights.len() {
                                            // Яркость неотрицательна; знак вклада задаётся градиентом оценки.
                                            gradients.pixel_weights[pixel_index] = GradientFiniteF32::try_from(
                                                gradients.pixel_weights[pixel_index].value()
                                                    + images_with_scores[input_pixels_index].pixels[pixel_index].value()
                                                        * score_gradient.value(),
                                            )?;
                                        }
                                        gradients.bias = GradientFiniteF32::try_from(gradients.bias.value() + score_gradient.value())?;
                                    }
                                }
                                gradients_by_digit
                            };

                            (mean_loss, pixel_weight_and_bias_gradients_by_digit)
                        };
                        // 9.5. Обновляем параметры Adam. Следующая группа увидит новые веса.
                        // Все градиенты рассчитаны по прежним весам; теперь обновляем веса и смещения Adam.
                        {
                            // Теперь все производные уже рассчитаны: можно менять веса.
                            // Обновляем единственный слой после завершения обратного прохода.
                            parameter_update_count += 1;
                            // Средние градиентов и их квадратов начинаются с нуля, поэтому первые значения занижены.
                            // Поправки: 1 минус 0.9 в степени числа обновлений и 1 минус 0.999 в той же степени.
                            let gradient_average_bias_correction: f32 =
                                1.0 - 0.9f32.powi(parameter_update_count);
                            let squared_gradient_average_bias_correction: f32 =
                                1.0 - 0.999f32.powi(parameter_update_count);

                            // Общая формула возвращает новое число; вызывающий код проверяет
                            // его как PixelWeightFiniteF32 или BiasFiniteF32. Типы этих параметров не смешиваются.
                            let calculate_updated_parameter =
                                |parameter: f32,
                                 moments: &mut AdamMoments,
                                 gradient: GradientFiniteF32|
                                 -> Result<f32, String> {
                                    moments.mean_gradient = GradientFiniteF32::try_from(
                                        0.9 * moments.mean_gradient.value()
                                            + 0.1 * gradient.value(),
                                    )?;
                                    moments.mean_squared_gradient = NonNegativeFiniteF32::try_from(
                                        0.999 * moments.mean_squared_gradient.value()
                                            + 0.001 * gradient.value() * gradient.value(),
                                    )?;
                                    Ok(parameter
                                        - learning_rate
                                            * (moments.mean_gradient.value()
                                                / gradient_average_bias_correction)
                                            / ((moments.mean_squared_gradient.value()
                                                / squared_gradient_average_bias_correction)
                                                .sqrt()
                                                + 1e-8))
                                };
                            // Типы переменных: digit_index: usize.
                            for digit_index in 0..pixel_weights_and_biases_by_digit.len() {
                                let pixel_weights_and_bias: &mut PixelWeightAndBias =
                                    &mut pixel_weights_and_biases_by_digit[digit_index];
                                let moments: &mut PixelWeightAndBiasAdamMoments =
                                    &mut adam_moments_by_digit[digit_index];
                                let gradients: &PixelWeightAndBiasGradients =
                                    &pixel_weight_and_bias_gradients_by_digit[digit_index];
                                // Типы переменных: pixel_index: usize.
                                for pixel_index in 0..pixel_weights_and_bias.pixel_weights.len() {
                                    pixel_weights_and_bias.pixel_weights[pixel_index] =
                                        PixelWeightFiniteF32::try_from(calculate_updated_parameter(
                                            pixel_weights_and_bias.pixel_weights[pixel_index]
                                                .value(),
                                            &mut moments.pixel_weights[pixel_index],
                                            gradients.pixel_weights[pixel_index],
                                        )?)?;
                                }
                                pixel_weights_and_bias.bias =
                                    BiasFiniteF32::try_from(calculate_updated_parameter(
                                        pixel_weights_and_bias.bias.value(),
                                        &mut moments.bias,
                                        gradients.bias,
                                    )?)?;
                            }
                        }
                        // Накопленная train loss относится к моментам обновлений: разные batch оценивались
                        // при разных весах. Умножаем среднюю ошибку группы на число её картинок для усреднения эпохи.
                        total_training_loss +=
                            mean_loss * batch_images_with_actual_digits_references.len() as f32;
                    }
                    total_training_loss
                };
                // Шаг 10. После каждой эпохи проверяем validation и сохраняем лучший вариант.
                // Затем возвращаемся к шагу 9, пока не пройдены все эпохи.
                // Validation — отложенные картинки: они помогают выбрать эпоху, но не меняют веса.
                // Падающая train loss при растущей validation loss может указывать на переобучение.
                let evaluation: PredictionEvaluation = measure_prediction_quality(
                    &pixel_weights_and_biases_by_digit,
                    &validation_images_with_actual_digits,
                )?;
                if !evaluation.mean_loss.is_finite() {
                    return Err("Validation loss не конечна".into());
                }
                // Запоминаем эпоху только при уменьшении validation loss (evaluation.mean_loss).
                // Выбор идёт по ошибке, а не по accuracy: уверенные неправильные ответы тоже важны.
                // Последняя эпоха не обязана быть лучшей, поэтому на test пойдут сохранённые best_pixel_weights_and_biases_by_digit.
                if evaluation.mean_loss < best_validation_loss {
                    best_validation_loss = evaluation.mean_loss;
                    best_epoch_number = epoch;
                    best_pixel_weights_and_biases_by_digit =
                        pixel_weights_and_biases_by_digit.clone();
                }
                println!(
                    "epoch={epoch} train_loss={:.5} validation_loss={:.5} validation_accuracy={:.4} elapsed={:.1}s",
                    total_training_loss / training_image_order.len() as f32,
                    evaluation.mean_loss,
                    evaluation.correct_prediction_fraction.value(),
                    started_at.elapsed().as_secs_f32()
                );
            }

            (best_pixel_weights_and_biases_by_digit, best_epoch_number)
        };
        (best_pixel_weights_and_biases_by_digit, best_epoch_number)
    };
    // Шаг 11. Один раз проверяем выбранную модель на test.
    // В таблице [7][1] — сколько семёрок приняты за единицы; [7][7] — верные семёрки.
    // Итоговая проверка: временные метрики и счётчики живут только в этом блоке.
    {
        // Итоговый отчёт на официальном test: используем выбранную по validation копию best_pixel_weights_and_biases_by_digit.
        // По test не выбираем веса, число эпох или скорость обучения.
        let evaluation: PredictionEvaluation = measure_prediction_quality(
            &best_pixel_weights_and_biases_by_digit,
            &test_images_with_actual_digits,
        )?;
        println!(
            "selected_epoch={best_epoch_number}; test={} loss={:.5} accuracy={:.4} elapsed={:.1}s",
            test_images_with_actual_digits.len(),
            evaluation.mean_loss,
            evaluation.correct_prediction_fraction.value(),
            started_at.elapsed().as_secs_f32()
        );
        println!("Ошибки[истинная цифра][прогноз]: оба измерения соответствуют цифрам 0..9");
        // Типы переменных: actual_digit: Digit.
        for actual_digit in ALL_DIGITS {
            let counts_for_actual_digit: [usize; 10] =
                ALL_DIGITS.map(|predicted_digit: Digit| -> usize {
                    evaluation
                        .digits_actual_and_predicted
                        .iter()
                        .filter(|prediction: &&DigitActualAndPredicted| -> bool {
                            prediction.actual_digit == actual_digit
                                && prediction.predicted_digit == predicted_digit
                        })
                        .count()
                });
            println!("{counts_for_actual_digit:?}");
        }
    }
    Ok(())
}
