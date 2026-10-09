fn main() -> Result<(), String> {
    // Урок 244. Учим линейную модель различать десять цифр по 784 пикселям.
    // Для каждого класса модель складывает «пиксель * его вес» и добавляет смещение.
    // Это даёт digit_scores — десять оценок (logits), пока ещё не вероятностей.
    // Softmax и cross-entropy показывают, насколько модель уверена в правильной цифре;
    // градиент говорит, как изменятся ошибки при небольшом изменении весов.
    // Adam обновляет веса по этим градиентам. Так повторяем много batch и эпох.
    //
    // Картинки обрабатываются группами: обычно по 64, последняя группа может быть меньше.
    // В матрице входов каждая строка содержит 784 яркости одной картинки.
    // Матрица весов связывает каждый пиксель с оценкой каждой из десяти цифр.
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
    // ndarray::Array1<f32> — вектор, ndarray::Array2<f32> — матрица; WeightsAndBiasesAsLinearLayer — структура с полями weights (веса) и biases (смещения).

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
    // WeightsAndBiasesAsLinearLayer хранит веса и смещения в именованных полях.
    // Такую же форму используем для градиентов и накопленных средних Adam.
    #[derive(Clone)]
    struct WeightsAndBiasesAsLinearLayer {
        weights: ndarray::Array2<f32>,
        biases: ndarray::Array1<f32>,
    }
    struct ForwardPass {
        input_pixels: ndarray::Array2<f32>,
        digit_scores: ndarray::Array2<f32>,
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
        |weights_and_biases_as_linear_layers: &[WeightsAndBiasesAsLinearLayer],
         input_pixels: &ndarray::Array2<f32>|
         -> ForwardPass {
            // input_pixels — матрица яркостей: одна строка на картинку, 784 столбца для пикселей.
            // weights — матрица весов: 784 строки для пикселей, 10 столбцов для цифр.
            // Каждый вес задаёт вклад яркости определённого пикселя в оценку определённой цифры.
            // biases — 10 обучаемых смещений, по одному на цифру.
            // Смещение цифры прибавляется к её оценке для каждой картинки независимо от пикселей.
            // digit_scores — матрица результатов: одна строка на картинку, 10 столбцов для оценок цифр.
            // Оценки пока не являются вероятностями.
            // Оценка цифры = сумма произведений яркостей пикселей на веса этой цифры + её смещение.
            // dot — матричное умножение; смещения прибавляются к каждой строке.
            let digit_scores: ndarray::Array2<f32> = input_pixels
                .dot(&weights_and_biases_as_linear_layers[0].weights)
                + &weights_and_biases_as_linear_layers[0].biases;
            ForwardPass {
                input_pixels: input_pixels.clone(),
                digit_scores,
            }
        };
    // Cross-entropy — среднее отрицательных натуральных логарифмов вероятностей правильных цифр.
    // Если правильному классу дана большая вероятность, ошибка мала; если малая — велика.
    // Вероятность правильной цифры 0.9 даёт ошибку около 0.105, а 0.1 — около 2.303.
    // Loss не равна доле неверных ответов: учитывает уверенность даже при правильном прогнозе.
    // Вычисляем среднюю ошибку и её производные по оценкам цифр для обновления весов.
    // В digit_scores одна строка на картинку и десять столбцов с оценками цифр.
    let calculate_cross_entropy_and_gradient = |digit_scores: &ndarray::Array2<f32>,
                                                actual_digits: &[Digit]|
     -> (f32, ndarray::Array2<f32>) {
        assert_eq!(digit_scores.nrows(), actual_digits.len());
        assert!(!actual_digits.is_empty());
        // Изменяемая копия scores нужна только вычислению вероятностей и производной.
        let (score_gradients, total_loss): (ndarray::Array2<f32>, f32) = {
            // Копия сначала содержит scores. По ходу цикла превращаем её в вероятности,
            // а затем в производные; исходные scores при этом остаются неизменными.
            let mut score_gradients: ndarray::Array2<f32> = digit_scores.clone();
            let mut total_loss: f32 = 0.0;
            // Типы переменных: scores_then_gradients_for_image: ndarray::ArrayViewMut1<'_, f32>, actual_digit: Digit.
            for (mut scores_then_gradients_for_image, &actual_digit) in
                score_gradients.rows_mut().into_iter().zip(actual_digits)
            {
                assert!((actual_digit as usize) < scores_then_gradients_for_image.len());
                // Softmax: экспоненту оценки каждой цифры делим на сумму экспонент всех десяти оценок.
                // Перед этим вычитаем максимальную оценку: вероятности сохраняются, экспоненты не переполняются.
                let maximum_score: f32 = scores_then_gradients_for_image
                    .iter()
                    .copied()
                    .fold(f32::NEG_INFINITY, f32::max);
                // Сохраняем исходный score правильного класса до преобразования строки.
                // actual_digit as usize выбирает столбец правильной цифры в digit_scores.
                let actual_digit_score: f32 =
                    scores_then_gradients_for_image[actual_digit as usize];
                // Строка теперь содержит экспоненты разностей оценок и максимальной оценки.
                scores_then_gradients_for_image
                    .mapv_inplace(|score: f32| -> f32 { (score - maximum_score).exp() });
                let exponential_sum: f32 = scores_then_gradients_for_image.sum();
                // Ошибка = максимальная оценка + логарифм суммы экспонент − оценка правильной цифры.
                // Так не нужно вычислять ln почти нулевой вероятности, которая могла округлиться до 0.
                total_loss += maximum_score + exponential_sum.ln() - actual_digit_score;
                // Делим экспоненты на их сумму: получаем вероятности, сумма которых равна 1.
                scores_then_gradients_for_image /= exponential_sum;
                // Производная для каждой цифры — её вероятность минус 1 для правильной цифры или минус 0 для остальных.
                // Например, вероятности [0.2, 0.8] при правильной цифре 0 дают производные [-0.8, 0.8].
                scores_then_gradients_for_image[actual_digit as usize] -= 1.0;
                // Учим по средней ошибке группы, поэтому делим производные на число картинок в ней.
                // Неполный последний batch тоже считается правильно. Повторно делить градиенты не надо.
                scores_then_gradients_for_image /= actual_digits.len() as f32;
            }
            (score_gradients, total_loss)
        };
        (total_loss / actual_digits.len() as f32, score_gradients)
    };
    // Группа для обучения и оценки: строка из 784 яркостей и правильная цифра для каждой картинки.
    // Индексы выбирают записи; нормализованные f64 пиксели переводим в f32 сети.
    let build_image_batch = |images_with_actual_digits: &[([f64; 784], Digit)],
                             batch_image_indices: &[usize]|
     -> (ndarray::Array2<f32>, Vec<Digit>) {
        (
            ndarray::Array2::from_shape_fn(
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
    let measure_prediction_quality =
        |weights_and_biases_as_linear_layers: &[WeightsAndBiasesAsLinearLayer],
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
                    let (input_pixels, actual_digits): (ndarray::Array2<f32>, Vec<Digit>) =
                        build_image_batch(images_with_actual_digits, batch_image_indices);
                    let forward_values: ForwardPass =
                        calculate_digit_scores(weights_and_biases_as_linear_layers, &input_pixels);
                    let digit_scores: &ndarray::Array2<f32> = &forward_values.digit_scores;
                    // loss одного batch — средняя: умножаем на его фактический размер.
                    // Затем делим общую сумму на число картинок, учитывая неполный последний batch.
                    total_loss +=
                        calculate_cross_entropy_and_gradient(digit_scores, &actual_digits).0
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
    let (best_weights_and_biases_as_linear_layers, best_epoch_number): (
        Vec<WeightsAndBiasesAsLinearLayer>,
        usize,
    ) = {
        println!(
            "Linear 784 -> 10: epochs={epoch_count}, batch={batch_size}, lr={learning_rate}, seed={random_seed}, data={}",
            dataset_directory.display()
        );
        // Производные по пикселям: производные по оценкам умножаем на транспонированную матрицу весов.
        // Производные по весам: транспонированную матрицу пикселей умножаем на производные по оценкам.
        // Производные по смещениям: складываем производные по оценкам всех картинок отдельно для каждой цифры.
        // Результат — (градиент по входу, градиент весов, градиент смещений).
        let calculate_linear_layer_gradients = |input_pixels: &ndarray::Array2<f32>,
                                                weights: &ndarray::Array2<f32>,
                                                score_gradients: &ndarray::Array2<f32>|
         -> (
            ndarray::Array2<f32>,
            ndarray::Array2<f32>,
            ndarray::Array1<f32>,
        ) {
            (
                score_gradients.dot(&weights.t()),
                input_pixels.t().dot(score_gradients),
                // Смещение одной цифры добавлялось каждой картинке, поэтому его производная
                // суммирует вклад всех строк (ось 0). Деление на число картинок уже учтено в cross-entropy.
                score_gradients.sum_axis(ndarray::Axis(0)),
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
        let (weights_and_biases_as_linear_layers, random_state): (
            Vec<WeightsAndBiasesAsLinearLayer>,
            u64,
        ) = {
            let mut random_state: u64 = random_state;
            // Создаём weights=[input_feature_count,output_digit_count] и biases=[output_digit_count]. Смещения начинают с нуля.
            // Веса начинаются с малых случайных значений по прежней формуле инициализации.
            let initialize_linear_layer = |input_feature_count: usize,
                                           output_digit_count: usize,
                                           random_state: &mut u64|
             -> WeightsAndBiasesAsLinearLayer {
                // Граница initial_weight_bound задаёт диапазон начальных весов.
                // Дисперсия этого распределения равна 2/input_feature_count.
                // Это масштаб He: для ReLU он помогает сохранять разумный размер сигналов в слоях.
                // Один и тот же способ инициализации здесь применяется ко всем матрицам весов.
                let initial_weight_bound: f32 = (6.0 / input_feature_count as f32).sqrt();
                WeightsAndBiasesAsLinearLayer {
                    weights: ndarray::Array2::from_shape_fn(
                        (input_feature_count, output_digit_count),
                        |_: (usize, usize)| -> f32 {
                            // Старшие 24 бита превращаем в random_fraction от 0 до 1; 2*random_fraction-1 даёт [-1,1).
                            // Умножение на initial_weight_bound задаёт диапазон начальных весов.
                            let random_fraction: f32 =
                                (next_random_number(random_state) >> 40) as f32 / 16777216.0;
                            (2.0 * random_fraction - 1.0) * initial_weight_bound
                        },
                    ),
                    biases: ndarray::Array1::zeros(output_digit_count),
                }
            };
            // Единственный слой: матрица из 784×10 весов и вектор из 10 смещений.
            let weights_and_biases_as_linear_layers: Vec<WeightsAndBiasesAsLinearLayer> =
                vec![initialize_linear_layer(784, 10, &mut random_state)];
            (weights_and_biases_as_linear_layers, random_state)
        };

        let (average_gradients, average_squared_gradients): (
            Vec<WeightsAndBiasesAsLinearLayer>,
            Vec<WeightsAndBiasesAsLinearLayer>,
        ) = {
            // Adam помнит историю отдельно для каждого веса и смещения.
            // average_gradients и average_squared_gradients изначально заполнены нулями.
            // Это сглаженные градиенты и их квадраты; формы совпадают с weights и biases.
            // Замыкание нужно только для начального создания этих массивов.
            let initialize_zero_averages = || -> Vec<WeightsAndBiasesAsLinearLayer> {
                weights_and_biases_as_linear_layers
                    .iter()
                    .map(|weights_and_biases_as_linear_layer: &WeightsAndBiasesAsLinearLayer| -> WeightsAndBiasesAsLinearLayer {
                        WeightsAndBiasesAsLinearLayer {
                            weights: ndarray::Array2::zeros(weights_and_biases_as_linear_layer.weights.dim()),
                            biases: ndarray::Array1::zeros(weights_and_biases_as_linear_layer.biases.dim()),
                        }
                    })
                    .collect::<Vec<WeightsAndBiasesAsLinearLayer>>()
            };
            (initialize_zero_averages(), initialize_zero_averages())
        };
        // Оцениваем ещё не обученную сеть: это точка отсчёта для сравнения.
        let initial_validation_loss: f32 = {
            let initial_evaluation: PredictionEvaluation = measure_prediction_quality(
                &weights_and_biases_as_linear_layers,
                &validation_images,
            );

            println!(
                "epoch=0 validation_loss={:.5} validation_accuracy={:.4}",
                initial_evaluation.mean_loss, initial_evaluation.accuracy
            );
            initial_evaluation.mean_loss
        };
        // Всё изменяемое состояние обучения живёт только в этом блоке эпох.
        // За его пределами доступны best_weights_and_biases_as_linear_layers и best_epoch_number без mut.
        let (best_weights_and_biases_as_linear_layers, best_epoch_number): (
            Vec<WeightsAndBiasesAsLinearLayer>,
            usize,
        ) = {
            let mut weights_and_biases_as_linear_layers: Vec<WeightsAndBiasesAsLinearLayer> =
                weights_and_biases_as_linear_layers;
            let mut training_image_order: Vec<usize> = training_image_order;
            let mut random_state: u64 = random_state;
            let mut average_gradients: Vec<WeightsAndBiasesAsLinearLayer> = average_gradients;
            let mut average_squared_gradients: Vec<WeightsAndBiasesAsLinearLayer> =
                average_squared_gradients;
            // Сохраняем отдельную копию лучших весов по validation loss.
            // Копия нужна, потому что следующие эпохи продолжат менять текущие weights_and_biases_as_linear_layers.
            // В начале лучший кандидат — ещё не обученная сеть (эпоха 0).
            let mut best_weights_and_biases_as_linear_layers: Vec<WeightsAndBiasesAsLinearLayer> =
                weights_and_biases_as_linear_layers.clone();
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
                        let (mean_loss, parameter_gradients): (
                            f32,
                            Vec<WeightsAndBiasesAsLinearLayer>,
                        ) = {
                            let (input_pixels, actual_digits): (ndarray::Array2<f32>, Vec<Digit>) =
                                build_image_batch(&train_images, batch_image_indices);
                            let forward_values: ForwardPass = calculate_digit_scores(
                                &weights_and_biases_as_linear_layers,
                                &input_pixels,
                            );
                            // Получаем среднюю ошибку группы и производные по оценкам цифр — начало обратного прохода.
                            // Ошибка должна быть конечной; NaN/∞ означают, что численный расчёт нарушился.
                            let (mean_loss, score_gradients): (f32, ndarray::Array2<f32>) =
                                calculate_cross_entropy_and_gradient(
                                    &forward_values.digit_scores,
                                    &actual_digits,
                                );
                            if !mean_loss.is_finite() {
                                return Err(
                                    "Ошибка обучения не конечна; уменьши learning-rate".into()
                                );
                            }
                            // Вычисляем производные по весам и смещениям; forward_values.input_pixels хранит входные яркости.
                            // Производную по самим входным пикселям не используем: картинки здесь не обучаются.
                            let (_, weight_gradients, bias_gradients): (
                                ndarray::Array2<f32>,
                                ndarray::Array2<f32>,
                                ndarray::Array1<f32>,
                            ) = calculate_linear_layer_gradients(
                                &forward_values.input_pixels,
                                &weights_and_biases_as_linear_layers[0].weights,
                                &score_gradients,
                            );
                            // Один слой — одна пара градиентов. Порядок совпадает с порядком weights_and_biases_as_linear_layers,
                            // поэтому цикл Adam ниже обновит правильную матрицу и её смещения.
                            let parameter_gradients: Vec<WeightsAndBiasesAsLinearLayer> =
                                vec![WeightsAndBiasesAsLinearLayer {
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
                            // Средние градиентов и их квадратов начинаются с нуля, поэтому первые значения занижены.
                            // Поправки: 1 минус 0.9 в степени числа обновлений и 1 минус 0.999 в той же степени.
                            let gradient_average_bias_correction: f32 =
                                1.0 - 0.9f32.powi(parameter_update_count);
                            let squared_gradient_average_bias_correction: f32 =
                                1.0 - 0.999f32.powi(parameter_update_count);
                            // Типы переменных: linear_layer_index: usize.
                            for linear_layer_index in 0..weights_and_biases_as_linear_layers.len() {
                                // Обновление весов: поле weights.
                                // Типы переменных: parameter: &mut f32, average_gradient: &mut f32, average_squared_gradient: &mut f32, parameter_gradient: f32.
                                for (
                                    ((parameter, average_gradient), average_squared_gradient),
                                    &parameter_gradient,
                                ) in weights_and_biases_as_linear_layers[linear_layer_index]
                                    .weights
                                    .iter_mut()
                                    .zip(average_gradients[linear_layer_index].weights.iter_mut())
                                    .zip(
                                        average_squared_gradients[linear_layer_index]
                                            .weights
                                            .iter_mut(),
                                    )
                                    .zip(parameter_gradients[linear_layer_index].weights.iter())
                                {
                                    // average_gradient хранит сглаженный градиент, включая его знак: 90% прежнего + 10% нового.
                                    // average_squared_gradient сглаживает квадрат градиента: 99.9% прежнего + 0.1% нового квадрата.
                                    *average_gradient =
                                        0.9 * *average_gradient + 0.1 * parameter_gradient;
                                    *average_squared_gradient = 0.999 * *average_squared_gradient
                                        + 0.001 * parameter_gradient * parameter_gradient;
                                    // Обновление Adam: из параметра вычитаем скорость обучения, умноженную на исправленный
                                    // средний градиент и делённую на корень исправленного среднего квадратов плюс 0.00000001.
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
                                ) in weights_and_biases_as_linear_layers[linear_layer_index]
                                    .biases
                                    .iter_mut()
                                    .zip(average_gradients[linear_layer_index].biases.iter_mut())
                                    .zip(
                                        average_squared_gradients[linear_layer_index]
                                            .biases
                                            .iter_mut(),
                                    )
                                    .zip(parameter_gradients[linear_layer_index].biases.iter())
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
                        // при разных весах. Умножаем среднюю ошибку группы на число её картинок для усреднения эпохи.
                        total_training_loss += mean_loss * batch_image_indices.len() as f32;
                    }
                    total_training_loss
                };
                // Validation — отложенные картинки: они помогают выбрать эпоху, но не меняют веса.
                // Падающая train loss при растущей validation loss может указывать на переобучение.
                let evaluation: PredictionEvaluation = measure_prediction_quality(
                    &weights_and_biases_as_linear_layers,
                    &validation_images,
                );
                if !evaluation.mean_loss.is_finite() {
                    return Err("Validation loss не конечна".into());
                }
                // Запоминаем эпоху только при уменьшении validation loss (evaluation.mean_loss).
                // Выбор идёт по ошибке, а не по accuracy: уверенные неправильные ответы тоже важны.
                // Последняя эпоха не обязана быть лучшей, поэтому на test пойдут сохранённые best_weights_and_biases_as_linear_layers.
                if evaluation.mean_loss < best_validation_loss {
                    best_validation_loss = evaluation.mean_loss;
                    best_epoch_number = epoch;
                    best_weights_and_biases_as_linear_layers =
                        weights_and_biases_as_linear_layers.clone();
                }
                println!(
                    "epoch={epoch} train_loss={:.5} validation_loss={:.5} validation_accuracy={:.4} elapsed={:.1}s",
                    total_training_loss / training_image_order.len() as f32,
                    evaluation.mean_loss,
                    evaluation.accuracy,
                    started_at.elapsed().as_secs_f32()
                );
            }

            (best_weights_and_biases_as_linear_layers, best_epoch_number)
        };
        (best_weights_and_biases_as_linear_layers, best_epoch_number)
    };
    // Итоговая проверка и вывод, как в 243: выбранная модель берётся из окружающего main.
    let evaluate_digit_predictions = |images_with_actual_digits: &[([f64; 784], Digit)]| -> () {
        // Итоговый отчёт на официальном test: используем выбранную по validation копию best_weights_and_biases_as_linear_layers.
        // По test не выбираем веса, число эпох или скорость обучения.
        let evaluation: PredictionEvaluation = measure_prediction_quality(
            &best_weights_and_biases_as_linear_layers,
            images_with_actual_digits,
        );
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
