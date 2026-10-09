fn main() -> Result<(), String> {
    // Урок 244. Учим линейную модель различать десять цифр по 784 пикселям.
    // Для каждого класса модель складывает «пиксель * его вес» и добавляет смещение.
    // Это даёт digit_scores — десять оценок (logits), пока ещё не вероятностей.
    // Softmax и cross-entropy показывают, насколько модель уверена в правильной цифре;
    // градиент говорит, как изменятся ошибки при небольшом изменении весов.
    // Adam обновляет веса по этим градиентам. Так повторяем много batch и эпох.
    //
    // Обозначения в комментариях: N — число изображений в batch, D — число входов,
    // K — число выходов; X — входы, W — веса, b — смещения, L — средняя ошибка.
    // [N,D] означает матрицу из N строк и D столбцов; T означает транспонирование.
    // Реальный N обычно равен 64, но последний batch может быть меньше.
    // Весь код остаётся в main: замыкания ниже объявляют операции, а вызываются
    // в блоках обучения и оценки. Из вложенных блоков выходят только нужные результаты.

    // При первом чтении следи за calculate_digit_scores, calculate_cross_entropy_and_gradient и циклом эпох.

    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя; Digit — метка цифры;
    // u64 — состояние генератора; String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 используется при чтении PNG, f32 — в вычислениях нейросети.
    // Array1<f32> — вектор, Array2<f32> — матрица; LinearLayer — структура с полями weights (веса) и biases (смещения).

    // Digit и порядок цифр общие для всех четырёх уроков MNIST.
    use mnist_data::{ALL_DIGITS, Digit, load_labeled_png_images};
    // Порядок соответствует папкам 0..9 и индексам классов в массивах.
    let all_digits: [Digit; 10] = ALL_DIGITS;

    // Засекаем время загрузки данных, обучения и итоговой оценки.
    let started_at: std::time::Instant = std::time::Instant::now();
    // Корень PNG фиксирован относительно папки урока; аргументы запуска не требуются.
    let dataset_directory: std::path::PathBuf =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
    // Для экспериментов меняй значения здесь и снова запускай cargo run.
    // epoch_count — полные проходы по train; batch_size — число картинок в одном обновлении.
    // learning_rate — размер шага Adam; random_seed — начальное состояние генератора.
    let epoch_count: usize = 15;
    let batch_size: usize = 64;
    let learning_rate: f32 = 0.001;
    let random_seed: u64 = 42;

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
    use ndarray::{Array1, Array2};
    // LinearLayer хранит веса и смещения в именованных полях.
    // Такую же форму используем для градиентов и накопленных средних Adam.
    #[derive(Clone)]
    struct LinearLayer {
        weights: Array2<f32>,
        biases: Array1<f32>,
    }
    struct ForwardPass {
        input_pixels: Array2<f32>,
        digit_scores: Array2<f32>,
    }
    struct DigitPrediction {
        actual_digit: Digit,
        predicted_digit: Digit,
    }
    struct PredictionEvaluation {
        mean_loss: f32,
        accuracy: f32,
        predictions: Vec<DigitPrediction>,
    }
    // Общие для обучения и оценки операции; их внутренние помощники локальны вызову.
    // Forward — прямой проход: из пикселей получаем оценки десяти цифр.
    // Сохраняем входы и оценки: они нужны для расчёта градиентов.
    let calculate_digit_scores =
        |model_layers: &[LinearLayer], input_pixels: &Array2<f32>| -> ForwardPass {
            // X=[N,784], W=[784,10], b=[10] -> scores=[N,10].
            // dot — матричное умножение; смещения прибавляются к каждой строке.
            let digit_scores: Array2<f32> =
                input_pixels.dot(&model_layers[0].weights) + &model_layers[0].biases;
            ForwardPass {
                input_pixels: input_pixels.clone(),
                digit_scores,
            }
        };
    // Считаем среднюю cross-entropy: L = mean(-ln(p_правильной_цифры)).
    // Если правильному классу дана большая вероятность, ошибка мала; если малая — велика.
    // Например, p_target=0.9 даёт loss≈0.105, а p_target=0.1 даёт loss≈2.303.
    // Loss не равна доле неверных ответов: учитывает уверенность даже при правильном прогнозе.
    // Здесь сразу вычисляем и L, и производную dL/dscores, нужную для обучения.
    // Форма scores — [N,K], где K=10 — число классов цифр.
    let calculate_cross_entropy_and_gradient =
        |digit_scores: &Array2<f32>, actual_digits: &[Digit]| -> (f32, Array2<f32>) {
            assert_eq!(digit_scores.nrows(), actual_digits.len());
            assert!(!actual_digits.is_empty());
            // Изменяемая копия scores нужна только вычислению вероятностей и производной.
            let (score_gradients, total_loss): (Array2<f32>, f32) = {
                // Копия сначала содержит scores. По ходу цикла превращаем её в вероятности,
                // а затем в производные; исходные scores при этом остаются неизменными.
                let mut score_gradients: Array2<f32> = digit_scores.clone();
                let mut total_loss: f32 = 0.0;
                // Типы переменных: scores_then_gradients_for_image: ndarray::ArrayViewMut1<'_, f32>, actual_digit: Digit.
                for (mut scores_then_gradients_for_image, &actual_digit) in
                    score_gradients.rows_mut().into_iter().zip(actual_digits)
                {
                    assert!((actual_digit as usize) < scores_then_gradients_for_image.len());
                    // Softmax: p_c = exp(score_c) / sum(exp(scores)). Вычитаем один максимум
                    // из всех scores: вероятности сохраняются, а экспоненты не переполняются.
                    let maximum_score: f32 = scores_then_gradients_for_image
                        .iter()
                        .copied()
                        .fold(f32::NEG_INFINITY, f32::max);
                    // Сохраняем исходный score правильного класса до преобразования строки.
                    // actual_digit as usize выбирает столбец правильной цифры в digit_scores.
                    let actual_digit_score: f32 =
                        scores_then_gradients_for_image[actual_digit as usize];
                    // Теперь в строке exp(score_c - max); это положительные ненормированные веса.
                    scores_then_gradients_for_image
                        .mapv_inplace(|score: f32| -> f32 { (score - maximum_score).exp() });
                    let exponential_sum: f32 = scores_then_gradients_for_image.sum();
                    // Это -ln(p_target), записанное как log-sum-exp - score_target.
                    // Так не нужно вычислять ln почти нулевой вероятности, которая могла округлиться до 0.
                    total_loss += maximum_score + exponential_sum.ln() - actual_digit_score;
                    // Делим экспоненты на их сумму: получаем вероятности, сумма которых равна 1.
                    scores_then_gradients_for_image /= exponential_sum;
                    // Для softmax вместе с cross-entropy производная равна p - one_hot(label).
                    // one_hot — строка с единицей у правильного класса и нулями у остальных.
                    // Пример: p=[0.2,0.8], правильный класс 0 -> производная [-0.8,0.8].
                    scores_then_gradients_for_image[actual_digit as usize] -= 1.0;
                    // Мы учим по СРЕДНЕЙ ошибке batch, поэтому делим производные на фактический N.
                    // Неполный последний batch тоже считается правильно. Повторно делить градиенты не надо.
                    scores_then_gradients_for_image /= actual_digits.len() as f32;
                }
                (score_gradients, total_loss)
            };
            (total_loss / actual_digits.len() as f32, score_gradients)
        };
    // Один batch для обучения и оценки: строки X=[N,784], метки actual_digits=[N].
    // Индексы выбирают записи; нормализованные f64 пиксели переводим в f32 сети.
    let build_image_batch = |images_with_actual_digits: &[([f64; 784], Digit)],
                             batch_image_indices: &[usize]|
     -> (Array2<f32>, Vec<Digit>) {
        (
            Array2::from_shape_fn(
                (batch_image_indices.len(), 784),
                |(image_position, pixel_index): (usize, usize)| -> f32 {
                    images_with_actual_digits[batch_image_indices[image_position]].0[pixel_index]
                        as f32
                },
            ),
            batch_image_indices
                .iter()
                .map(|&image_index: &usize| -> Digit { images_with_actual_digits[image_index].1 })
                .collect::<Vec<Digit>>(),
        )
    };

    // Оценка читает текущую модель, не обновляя веса. Передаём целый готовый набор.
    // Модель остаётся параметром: validation проверяет разные веса после каждой эпохи.
    let measure_prediction_quality = |model_layers: &[LinearLayer],
                                      images_with_actual_digits: &[([f64; 784], Digit)]|
     -> PredictionEvaluation {
        // Наибольшая оценка выбирает цифру; при равенстве предпочитаем меньшую.
        let digit_with_highest_score =
            |digit_scores_for_image: ndarray::ArrayView1<'_, f32>| -> Digit {
                let predicted_digit_index: usize = (0..digit_scores_for_image.len())
                    .max_by(
                        |&first_digit_index: &usize,
                         &second_digit_index: &usize|
                         -> std::cmp::Ordering {
                            digit_scores_for_image[first_digit_index]
                                .total_cmp(&digit_scores_for_image[second_digit_index])
                                .then_with(|| -> std::cmp::Ordering {
                                    second_digit_index.cmp(&first_digit_index)
                                })
                        },
                    )
                    .unwrap();
                all_digits[predicted_digit_index]
            };
        assert!(!images_with_actual_digits.is_empty() && batch_size > 0);
        // Индексы здесь нужны только сборке batch; снаружи передаётся сам набор картинок.
        let image_indices: Vec<usize> = (0..images_with_actual_digits.len()).collect();
        let (predictions, total_loss): (Vec<DigitPrediction>, f32) = {
            let mut predictions: Vec<DigitPrediction> =
                Vec::with_capacity(images_with_actual_digits.len());
            let mut total_loss: f32 = 0.0;
            // Типы переменных: batch_image_indices: &[usize].
            for batch_image_indices in image_indices.chunks(batch_size) {
                let (input_pixels, actual_digits): (Array2<f32>, Vec<Digit>) =
                    build_image_batch(images_with_actual_digits, batch_image_indices);
                let forward_values: ForwardPass =
                    calculate_digit_scores(model_layers, &input_pixels);
                let digit_scores: &Array2<f32> = &forward_values.digit_scores;
                // loss одного batch — средняя: умножаем на его фактический размер.
                // Затем делим общую сумму на число картинок, учитывая неполный последний batch.
                total_loss += calculate_cross_entropy_and_gradient(digit_scores, &actual_digits).0
                    * batch_image_indices.len() as f32;
                // Типы переменных: digit_scores_for_image: ndarray::ArrayView1<'_, f32>, actual_digit: Digit.
                for (digit_scores_for_image, actual_digit) in
                    digit_scores.rows().into_iter().zip(actual_digits)
                {
                    predictions.push(DigitPrediction {
                        actual_digit,
                        predicted_digit: digit_with_highest_score(digit_scores_for_image),
                    });
                }
            }
            (predictions, total_loss)
        };
        let correct_prediction_count: usize = predictions
            .iter()
            .filter(|prediction: &&DigitPrediction| -> bool {
                prediction.actual_digit == prediction.predicted_digit
            })
            .count();
        PredictionEvaluation {
            mean_loss: total_loss / images_with_actual_digits.len() as f32,
            accuracy: correct_prediction_count as f32 / images_with_actual_digits.len() as f32,
            predictions,
        }
    };
    // Из обучения выходят только выбранные веса и необходимые для отчёта значения.
    let (best_model_layers, best_epoch_number): (Vec<LinearLayer>, usize) = {
        println!(
            "Linear 784 -> 10: epochs={epoch_count}, batch={batch_size}, lr={learning_rate}, seed={random_seed}, data={}",
            dataset_directory.display()
        );
        // Для Z = XW+b и входящего G=dL/dZ правило цепочки даёт:
        // dL/dX = G W^T, dL/dW = X^T G, dL/db = сумма строк G.
        // Формы: X=[N,D], W=[D,K], G=[N,K]; результаты [N,D], [D,K], [K].
        // Результат — (градиент по входу, градиент весов, градиент смещений).
        let calculate_linear_layer_gradients = |input_pixels: &Array2<f32>,
                                                weights: &Array2<f32>,
                                                score_gradients: &Array2<f32>|
         -> (Array2<f32>, Array2<f32>, Array1<f32>) {
            use ndarray::Axis;
            (
                score_gradients.dot(&weights.t()),
                input_pixels.t().dot(score_gradients),
                // Одно смещение b_k добавлялось каждой строке, поэтому его производная
                // суммирует вклад всех строк (ось 0). Усреднение по N уже учтено в cross-entropy.
                score_gradients.sum_axis(Axis(0)),
            )
        };

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
        let shuffle_training_indices =
            |image_indices: &mut [usize], random_state: &mut u64| -> () {
                // Типы переменных: current_position: usize.
                for current_position in (1..image_indices.len()).rev() {
                    let random_position: usize =
                        (next_random_number(random_state) % (current_position as u64 + 1)) as usize;
                    image_indices.swap(current_position, random_position);
                }
            };
        // Перемешиваем позиции готового train. Порядок RNG совпадает с прежним уроком:
        // сначала одно перемешивание, затем инициализация весов, затем эпохи.
        let (training_image_order, random_state): (Vec<usize>, u64) = {
            let mut training_image_order: Vec<usize> = (0..train_images.len()).collect();
            let mut random_state: u64 = random_seed;
            shuffle_training_indices(&mut training_image_order, &mut random_state);
            (training_image_order, random_state)
        };
        // Проверяем все десять цифр, не используя validation или test.
        if all_digits.into_iter().any(|digit: Digit| -> bool {
            !train_images
                .iter()
                .any(|image: &([f64; 784], Digit)| -> bool { image.1 == digit })
        }) {
            return Err("Train должен содержать примеры всех десяти цифр".into());
        }
        println!(
            "train={}, validation={}; split=every fifth per class, sorted PNG filenames",
            train_images.len(),
            validation_images.len()
        );
        let (model_layers, random_state): (Vec<LinearLayer>, u64) = {
            let mut random_state: u64 = random_state;
            // Создаём weights=[input_feature_count,output_digit_count] и biases=[output_digit_count]. Смещения начинают с нуля.
            // Веса начинаются с малых случайных значений по прежней формуле инициализации.
            let initialize_linear_layer = |input_feature_count: usize,
                                           output_digit_count: usize,
                                           random_state: &mut u64|
             -> LinearLayer {
                // Граница initial_weight_bound задаёт диапазон начальных весов.
                // Дисперсия этого распределения равна 2/input_feature_count.
                // Это масштаб He: для ReLU он помогает сохранять разумный размер сигналов в слоях.
                // Один и тот же способ инициализации здесь применяется ко всем матрицам весов.
                let initial_weight_bound: f32 = (6.0 / input_feature_count as f32).sqrt();
                LinearLayer {
                    weights: Array2::from_shape_fn(
                        (input_feature_count, output_digit_count),
                        |_: (usize, usize)| -> f32 {
                            // Старшие 24 бита превращаем в random_fraction от 0 до 1; 2*random_fraction-1 даёт [-1,1).
                            // Умножение на initial_weight_bound задаёт диапазон начальных весов.
                            let random_fraction: f32 =
                                (next_random_number(random_state) >> 40) as f32 / 16777216.0;
                            (2.0 * random_fraction - 1.0) * initial_weight_bound
                        },
                    ),
                    biases: Array1::zeros(output_digit_count),
                }
            };
            // Единственный слой: 784 входа и 10 классов; W=[784,10], b=[10].
            let model_layers: Vec<LinearLayer> =
                vec![initialize_linear_layer(784, 10, &mut random_state)];
            (model_layers, random_state)
        };

        let (average_gradients, average_squared_gradients): (Vec<LinearLayer>, Vec<LinearLayer>) = {
            // Adam помнит историю отдельно для каждого веса и смещения.
            // average_gradients и average_squared_gradients изначально заполнены нулями.
            // Это сглаженные градиенты и их квадраты; формы совпадают с weights и biases.
            // Замыкание нужно только для начального создания этих массивов.
            let initialize_zero_averages = || -> Vec<LinearLayer> {
                model_layers
                    .iter()
                    .map(|layer: &LinearLayer| -> LinearLayer {
                        LinearLayer {
                            weights: Array2::zeros(layer.weights.dim()),
                            biases: Array1::zeros(layer.biases.dim()),
                        }
                    })
                    .collect::<Vec<LinearLayer>>()
            };
            (initialize_zero_averages(), initialize_zero_averages())
        };
        // Оцениваем ещё не обученную сеть: это точка отсчёта для сравнения.
        let initial_validation_loss: f32 = {
            let initial_evaluation: PredictionEvaluation =
                measure_prediction_quality(&model_layers, &validation_images);

            println!(
                "epoch=0 validation_loss={:.5} validation_accuracy={:.4}",
                initial_evaluation.mean_loss, initial_evaluation.accuracy
            );
            initial_evaluation.mean_loss
        };
        // Всё изменяемое состояние обучения живёт только в этом блоке эпох.
        // За его пределами доступны best_model_layers и best_epoch_number без mut.
        let (best_model_layers, best_epoch_number): (Vec<LinearLayer>, usize) = {
            let mut model_layers: Vec<LinearLayer> = model_layers;
            let mut training_image_order: Vec<usize> = training_image_order;
            let mut random_state: u64 = random_state;
            let mut average_gradients: Vec<LinearLayer> = average_gradients;
            let mut average_squared_gradients: Vec<LinearLayer> = average_squared_gradients;
            // Сохраняем отдельную копию лучших весов по validation loss.
            // Копия нужна, потому что следующие эпохи продолжат менять текущие model_layers.
            // В начале лучший кандидат — ещё не обученная сеть (эпоха 0).
            let mut best_model_layers: Vec<LinearLayer> = model_layers.clone();
            let mut best_validation_loss: f32 = initial_validation_loss;
            let mut best_epoch_number: usize = 0;
            // parameter_update_count — номер ОБНОВЛЕНИЯ весов, не эпохи. Он увеличивается после каждого batch
            // и нужен Adam для поправки на нулевые начальные средние.
            let mut parameter_update_count: i32 = 0i32;
            // Одна эпоха: пройти все batch train, затем оценить validation без обновления весов.
            // Test подготовлен заранее, но до выбора лучшей эпохи не участвует в вычислениях.
            // Типы переменных: epoch: usize.
            for epoch in 1..=epoch_count {
                // Меняем только порядок обучающих индексов. Генератор продолжает своё состояние,
                // поэтому каждая эпоха имеет новый, но воспроизводимый при том же seed порядок.
                shuffle_training_indices(&mut training_image_order, &mut random_state);
                // Сумма меняется только при обходе train; для отчёта возвращаем готовую train loss.
                let total_training_loss: f32 = {
                    let mut total_training_loss: f32 = 0.0;
                    // Типы переменных: batch_image_indices: &[usize].
                    for batch_image_indices in training_image_order.chunks(batch_size) {
                        // Сначала весь прямой и обратный проход по СТАРЫМ весам.
                        // Наружу из этого блока выходят только ошибка batch и градиенты параметров;
                        // входы, оценки цифр и промежуточные производные остаются внутри.
                        let (mean_loss, parameter_gradients): (f32, Vec<LinearLayer>) = {
                            let (input_pixels, actual_digits): (Array2<f32>, Vec<Digit>) =
                                build_image_batch(&train_images, batch_image_indices);
                            let forward_values: ForwardPass =
                                calculate_digit_scores(&model_layers, &input_pixels);
                            // Получаем среднюю ошибку batch и dL/dscores — начало обратного прохода.
                            // Ошибка должна быть конечной; NaN/∞ означают, что численный расчёт нарушился.
                            let (mean_loss, score_gradients): (f32, Array2<f32>) =
                                calculate_cross_entropy_and_gradient(
                                    &forward_values.digit_scores,
                                    &actual_digits,
                                );
                            if !mean_loss.is_finite() {
                                return Err(
                                    "Ошибка обучения не конечна; уменьши learning-rate".into()
                                );
                            }
                            // Для единственного слоя получаем dL/dW и dL/db. forward_values.input_pixels — сохранённый X.
                            // Производную по самим входным пикселям не используем: картинки здесь не обучаются.
                            let (_, weight_gradients, bias_gradients): (
                                Array2<f32>,
                                Array2<f32>,
                                Array1<f32>,
                            ) = calculate_linear_layer_gradients(
                                &forward_values.input_pixels,
                                &model_layers[0].weights,
                                &score_gradients,
                            );
                            // Один слой — одна пара градиентов. Порядок совпадает с порядком model_layers,
                            // поэтому цикл Adam ниже обновит правильную матрицу и её смещения.
                            let parameter_gradients: Vec<LinearLayer> = vec![LinearLayer {
                                weights: weight_gradients,
                                biases: bias_gradients,
                            }];

                            (mean_loss, parameter_gradients)
                        };
                        // Все градиенты рассчитаны по прежним весам; теперь обновляем веса и смещения Adam.
                        {
                            // Теперь все производные уже рассчитаны: можно менять веса.
                            // Обновляем каждый слой по его градиенту, не смешивая новые веса со старым backward.
                            parameter_update_count += 1;
                            // В начале m и v равны нулю, поэтому первые средние занижены.
                            // Деление на 1-β^step исправляет это смещение: β₁=0.9, β₂=0.999.
                            let gradient_average_bias_correction: f32 =
                                1.0 - 0.9f32.powi(parameter_update_count);
                            let squared_gradient_average_bias_correction: f32 =
                                1.0 - 0.999f32.powi(parameter_update_count);
                            // Типы переменных: layer_index: usize.
                            for layer_index in 0..model_layers.len() {
                                // Обновление весов: поле weights.
                                // Типы переменных: parameter: &mut f32, average_gradient: &mut f32, average_squared_gradient: &mut f32, parameter_gradient: f32.
                                for (
                                    ((parameter, average_gradient), average_squared_gradient),
                                    &parameter_gradient,
                                ) in model_layers[layer_index]
                                    .weights
                                    .iter_mut()
                                    .zip(average_gradients[layer_index].weights.iter_mut())
                                    .zip(average_squared_gradients[layer_index].weights.iter_mut())
                                    .zip(parameter_gradients[layer_index].weights.iter())
                                {
                                    // average_gradient хранит сглаженный градиент, включая его знак: 90% прежнего + 10% нового.
                                    // average_squared_gradient сглаживает квадрат градиента: 99.9% прежнего + 0.1% нового квадрата.
                                    *average_gradient =
                                        0.9 * *average_gradient + 0.1 * parameter_gradient;
                                    *average_squared_gradient = 0.999 * *average_squared_gradient
                                        + 0.001 * parameter_gradient * parameter_gradient;
                                    // Adam: parameter -= learning_rate * m_hat / (sqrt(v_hat)+epsilon).
                                    // Вычитаем направление градиента, чтобы уменьшать ошибку; большой накопленный
                                    // масштаб градиента уменьшает относительный шаг. epsilon=1e-8 защищает от деления на 0.
                                    *parameter -= learning_rate
                                        * (*average_gradient / gradient_average_bias_correction)
                                        / ((*average_squared_gradient
                                            / squared_gradient_average_bias_correction)
                                            .sqrt()
                                            + 1e-8);
                                }
                                // Та же формула Adam для смещений: поле biases.
                                // Типы переменных: parameter: &mut f32, average_gradient: &mut f32, average_squared_gradient: &mut f32, parameter_gradient: f32.
                                for (
                                    ((parameter, average_gradient), average_squared_gradient),
                                    &parameter_gradient,
                                ) in model_layers[layer_index]
                                    .biases
                                    .iter_mut()
                                    .zip(average_gradients[layer_index].biases.iter_mut())
                                    .zip(average_squared_gradients[layer_index].biases.iter_mut())
                                    .zip(parameter_gradients[layer_index].biases.iter())
                                {
                                    *average_gradient =
                                        0.9 * *average_gradient + 0.1 * parameter_gradient;
                                    *average_squared_gradient = 0.999 * *average_squared_gradient
                                        + 0.001 * parameter_gradient * parameter_gradient;

                                    *parameter -= learning_rate
                                        * (*average_gradient / gradient_average_bias_correction)
                                        / ((*average_squared_gradient
                                            / squared_gradient_average_bias_correction)
                                            .sqrt()
                                            + 1e-8);
                                }
                            }
                        }
                        // Накопленная train loss относится к моментам обновлений: разные batch оценивались
                        // при разных весах. Умножение на N даёт сумму ошибок, чтобы правильно усреднить эпоху.
                        total_training_loss += mean_loss * batch_image_indices.len() as f32;
                    }
                    total_training_loss
                };
                // Validation — отложенные картинки: они помогают выбрать эпоху, но не меняют веса.
                // Падающая train loss при растущей validation loss может указывать на переобучение.
                let evaluation: PredictionEvaluation =
                    measure_prediction_quality(&model_layers, &validation_images);
                if !evaluation.mean_loss.is_finite() {
                    return Err("Validation loss не конечна".into());
                }
                // Запоминаем эпоху только при уменьшении validation loss (evaluation.mean_loss).
                // Выбор идёт по ошибке, а не по accuracy: уверенные неправильные ответы тоже важны.
                // Последняя эпоха не обязана быть лучшей, поэтому на test пойдут сохранённые best_model_layers.
                if evaluation.mean_loss < best_validation_loss {
                    best_validation_loss = evaluation.mean_loss;
                    best_epoch_number = epoch;
                    best_model_layers = model_layers.clone();
                }
                println!(
                    "epoch={epoch} train_loss={:.5} validation_loss={:.5} validation_accuracy={:.4} elapsed={:.1}s",
                    total_training_loss / training_image_order.len() as f32,
                    evaluation.mean_loss,
                    evaluation.accuracy,
                    started_at.elapsed().as_secs_f32()
                );
            }

            (best_model_layers, best_epoch_number)
        };
        (best_model_layers, best_epoch_number)
    };
    // Итоговая проверка и вывод, как в 243: выбранная модель берётся из окружающего main.
    let evaluate_digit_predictions = |images_with_actual_digits: &[([f64; 784], Digit)]| -> () {
        // Итоговый отчёт на официальном test: используем выбранную по validation копию best_model_layers.
        // По test не выбираем веса, число эпох или скорость обучения.
        let evaluation: PredictionEvaluation =
            measure_prediction_quality(&best_model_layers, images_with_actual_digits);
        println!(
            "selected_epoch={best_epoch_number}; test={} loss={:.5} accuracy={:.4} elapsed={:.1}s",
            images_with_actual_digits.len(),
            evaluation.mean_loss,
            evaluation.accuracy,
            started_at.elapsed().as_secs_f32()
        );
        println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
        // Типы переменных: actual_digit: Digit.
        for actual_digit in all_digits {
            let counts_for_actual_digit: Vec<usize> = all_digits
                .into_iter()
                .map(|predicted_digit: Digit| -> usize {
                    evaluation
                        .predictions
                        .iter()
                        .filter(|prediction: &&DigitPrediction| -> bool {
                            prediction.actual_digit == actual_digit
                                && prediction.predicted_digit == predicted_digit
                        })
                        .count()
                })
                .collect();
            println!("{counts_for_actual_digit:?}");
        }
    };
    evaluate_digit_predictions(&test_images);
    Ok(())
}
