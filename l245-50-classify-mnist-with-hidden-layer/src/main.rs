fn main() -> Result<(), String> {
    // Урок 245. Нейросеть: 784 пикселя -> 128 скрытых нейронов -> ReLU -> 10 scores.
    // Скрытый нейрон — взвешенная сумма входов плюс своё смещение.
    // ReLU оставляет положительные значения и заменяет остальные нулём.
    // Это нелинейность: сеть может учить более сложные сочетания пикселей.
    // Оба слоя обучаются. Ошибку передаём назад от выхода к скрытому слою и к весам.
    //
    // N — число картинок в batch; X — входы, W — веса, b — смещения, L — ошибка.
    // Форма [N,784]: первое измерение выбирает картинку, второе — пиксель.
    // Знак T означает транспонирование: первое и второе измерения меняются местами.
    // В слоях используем f32 — числа с плавающей точкой, достаточные для этого опыта.
    // Сначала объявляем локальные операции, затем main выполняет подготовку,
    // обучение, выбор лучшей эпохи по validation и одну итоговую оценку на test.

    // При первом чтении следи за forward, cross_entropy и циклом for epoch.

    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя; Digit — метка цифры;
    // u64 — состояние генератора; String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 используется при чтении PNG, f32 — в вычислениях нейросети.
    // ExampleFeatures — признаки одного примера; InputFeatureWeights — связи одного входа с выходными нейронами.

    // Digit и порядок цифр общие для всех четырёх уроков MNIST.
    // Порядок соответствует папкам 0..9 и индексам классов в массивах.
    use mnist_data::{
        ALL_DIGITS, Digit, NormalizedPixelBrightnessF64, load_mnist_images, mnist_dataset_directory,
    };

    // Засекаем время загрузки данных, обучения и итоговой оценки.
    let start: std::time::Instant = std::time::Instant::now();
    // Общий путь к PNG задаётся в mnist_data; аргументы запуска не требуются.
    let data: std::path::PathBuf = mnist_dataset_directory();
    // Для экспериментов меняй значения здесь и снова запускай cargo run.
    // epochs — полные проходы по train; batch_size — число картинок в одном обновлении.
    // learning_rate — размер шага Adam; seed — начальное состояние генератора.
    let epochs: usize = 15;
    let batch_size: usize = 64;
    let learning_rate: f32 = 0.001;
    let seed: u64 = 42;

    // PNG читает общий загрузчик из mnist_data: сортировка, проверка формата и нормализация.

    // Один пример — картинка, набор признаков нейронов или одно окно свёртки.
    // Внешний Vec<ExampleFeatures> перечисляет примеры; features — признаки одного примера.
    #[derive(Clone)]
    struct ExampleFeatures {
        features: Vec<f32>,
    }
    // Один входной признак связан с каждым выходным нейроном своим весом.
    #[derive(Clone)]
    struct InputFeatureWeights {
        output_neurons: Vec<f32>,
    }

    // Числовая часть имеет свою область видимости для массивов и замыканий.
    {
        // Layer хранит веса входных признаков и смещения выходных нейронов в именованных полях.
        // Для Dense W имеет форму [число входов, число выходов], b — [число выходов].
        #[derive(Clone)]
        struct Layer {
            weights: Vec<InputFeatureWeights>,
            biases: Vec<f32>,
        }
        struct ForwardPass {
            input_pixels: Vec<ExampleFeatures>,
            hidden_neurons: Vec<ExampleFeatures>,
            digit_scores: Vec<ExampleFeatures>,
        }
        // Общие для обучения и оценки операции; их внутренние помощники локальны вызову.
        // Forward — прямой проход: из пикселей получаем оценки десяти цифр.
        // Возвращаем также промежуточные значения, которые понадобятся backward.
        // Именованные поля ForwardPass сохраняют вход, скрытые признаки и оценки цифр.
        let forward = |layers: &[Layer], input: &Vec<ExampleFeatures>| -> ForwardPass {
            // ReLU(x) = max(x,0). Она «выключает» отрицательные ответы нейронов.
            // Без нелинейности два последовательных Dense можно было бы заменить одним.
            let dense = |input: &Vec<ExampleFeatures>, layer: &Layer| -> Vec<ExampleFeatures> {
                let mut output: Vec<ExampleFeatures> = vec![
                    ExampleFeatures {
                        features: vec![0.0; layer.biases.len()]
                    };
                    input.len()
                ];
                // Типы переменных: input_index: usize.
                for input_index in 0..input.len() {
                    assert_eq!(input[input_index].features.len(), layer.weights.len());
                    // Типы переменных: weights_index: usize.
                    for weights_index in 0..layer.weights.len() {
                        // Типы переменных: biases_index: usize.
                        for biases_index in 0..layer.biases.len() {
                            output[input_index].features[biases_index] += input[input_index]
                                .features[weights_index]
                                * layer.weights[weights_index].output_neurons[biases_index];
                        }
                    }
                    // Типы переменных: biases_index: usize.
                    for biases_index in 0..layer.biases.len() {
                        output[input_index].features[biases_index] += layer.biases[biases_index];
                    }
                }
                output
            };
            let relu = |values: &Vec<ExampleFeatures>| -> Vec<ExampleFeatures> {
                let mut output: Vec<ExampleFeatures> = values.clone();
                // Типы переменных: example_values: &mut ExampleFeatures.
                for example_values in &mut output {
                    // Типы переменных: value: &mut f32.
                    for value in &mut example_values.features {
                        *value = value.max(0.0);
                    }
                }
                output
            };

            // Первый Dense: [N,784] * [784,128] + [128] -> [N,128], затем ReLU.
            // 128 признаков учатся совместно с выходным слоем, а не задаются вручную.
            let hidden: Vec<ExampleFeatures> = relu(&dense(input, &layers[0]));
            // Второй Dense: [N,128] * [128,10] + [10] -> [N,10].
            // Последний слой оставляем линейным: softmax включён в расчёт loss ниже.
            let scores: Vec<ExampleFeatures> = dense(&hidden, &layers[1]);
            ForwardPass {
                input_pixels: input.clone(),
                hidden_neurons: hidden,
                digit_scores: scores,
            }
        };
        // Считаем среднюю cross-entropy: L = mean(-ln(p_правильной_цифры)).
        // Если правильному классу дана большая вероятность, ошибка мала; если малая — велика.
        // Например, p_target=0.9 даёт loss≈0.105, а p_target=0.1 даёт loss≈2.303.
        // Loss не равна доле неверных ответов: учитывает уверенность даже при верном argmax.
        // Здесь сразу вычисляем и L, и производную dL/dscores, нужную для обучения.
        // Форма scores — [N,K], где K=10 — число классов цифр.
        let cross_entropy =
            |scores: &Vec<ExampleFeatures>, labels: &[Digit]| -> (f32, Vec<ExampleFeatures>) {
                assert_eq!(scores.len(), labels.len());
                assert!(!labels.is_empty());
                // Изменяемая копия scores нужна только вычислению вероятностей и производной.
                let (gradient, loss): (Vec<ExampleFeatures>, f32) = {
                    // Копия сначала содержит scores. По ходу цикла превращаем её в вероятности,
                    // а затем в производные; исходные scores при этом остаются неизменными.
                    let mut gradient: Vec<ExampleFeatures> = scores.clone();
                    let mut loss: f32 = 0.0;
                    // Типы переменных: example_values: &mut ExampleFeatures, label: Digit.
                    for (example_values, &label) in gradient.iter_mut().zip(labels) {
                        assert!((label as usize) < example_values.features.len());
                        // Softmax: p_c = exp(score_c) / sum(exp(scores)). Вычитаем один максимум
                        // из всех scores: вероятности сохраняются, а экспоненты не переполняются.
                        let max: f32 = example_values
                            .features
                            .iter()
                            .copied()
                            .fold(f32::NEG_INFINITY, f32::max);
                        // Сохраняем исходный score правильного класса до преобразования значений примера.
                        // label — вариант Digit; label as usize выбирает значение этой цифры во втором измерении scores.
                        let target: f32 = example_values.features[label as usize];
                        // Теперь значения примера равны exp(score_c - max); это положительные ненормированные веса.
                        // Типы переменных: value: &mut f32.
                        for value in example_values.features.iter_mut() {
                            *value = (*value - max).exp();
                        }
                        let sum: f32 = example_values.features.iter().sum();
                        // Это -ln(p_target), записанное как log-sum-exp - score_target.
                        // Так не нужно вычислять ln почти нулевой вероятности, которая могла округлиться до 0.
                        loss += max + sum.ln() - target;
                        // Делим экспоненты на их сумму: получаем вероятности, сумма которых равна 1.
                        // Типы переменных: value: &mut f32.
                        for value in example_values.features.iter_mut() {
                            *value /= sum;
                        }
                        // Для softmax вместе с cross-entropy производная равна p - one_hot(label).
                        // one_hot — массив с единицей у правильного класса и нулями у остальных.
                        // Пример: p=[0.2,0.8], правильный класс 0 -> производная [-0.8,0.8].
                        example_values.features[label as usize] -= 1.0;
                        // Мы учим по СРЕДНЕЙ ошибке batch, поэтому делим производные на фактический N.
                        // Неполный последний batch тоже считается правильно. Повторно делить градиенты не надо.
                        // Типы переменных: value: &mut f32.
                        for value in example_values.features.iter_mut() {
                            *value /= labels.len() as f32;
                        }
                    }
                    (gradient, loss)
                };
                (loss / labels.len() as f32, gradient)
            };
        // Один batch: X[картинка][пиксель] имеет размеры [N,784], а labels[картинка] — [N].
        // Индексы выбирают записи; нормализованные f64 пиксели переводим в f32 сети.
        let batch = |digits: &[([NormalizedPixelBrightnessF64; 784], Digit)],
                     examples: &[usize]|
         -> (Vec<ExampleFeatures>, Vec<Digit>) {
            (
                {
                    let first_dimension_length: usize = examples.len();
                    let second_dimension_length: usize = 784;
                    let mut values: Vec<ExampleFeatures> = vec![
                        ExampleFeatures {
                            features: vec![0.0; second_dimension_length]
                        };
                        first_dimension_length
                    ];
                    // Типы переменных: first_dimension_index: usize.
                    for first_dimension_index in 0..first_dimension_length {
                        // Типы переменных: second_dimension_index: usize.
                        for second_dimension_index in 0..second_dimension_length {
                            let (examples_index, pixels_index): (usize, usize) =
                                (first_dimension_index, second_dimension_index);
                            values[first_dimension_index].features[second_dimension_index] =
                                digits[examples[examples_index]].0[pixels_index].value() as f32;
                        }
                    }
                    values
                },
                examples
                    .iter()
                    .map(|&digits_index: &usize| -> Digit { digits[digits_index].1 })
                    .collect::<Vec<Digit>>(),
            )
        };

        // Оценка только читает веса: не передаёт производные назад через слои и не обновляет параметры.
        // Возвращаем (средняя loss, доля верных ответов, матрица ошибок).
        // Это же правило используем для validation и итогового test.
        let evaluate = |layers: &[Layer],
                        digits: &[([NormalizedPixelBrightnessF64; 784], Digit)],
                        examples: &[usize],
                        batch_size: usize|
         -> (f32, f32, [[usize; 10]; 10]) {
            // Находим индекс наибольшего score в массиве оценок одной картинки и возвращаем Digit.
            // Softmax сохраняет порядок scores, поэтому для выбора класса вероятности не нужны.
            // При одинаковых scores берём меньший индекс: результат однозначен.
            let argmax = |example_values: &[f32]| -> Digit {
                let scores_index: usize = (0..example_values.len())
                    .max_by(
                        |&first_scores_index: &usize,
                         &second_scores_index: &usize|
                         -> std::cmp::Ordering {
                            example_values[first_scores_index]
                                .total_cmp(&example_values[second_scores_index])
                                .then_with(|| -> std::cmp::Ordering {
                                    second_scores_index.cmp(&first_scores_index)
                                })
                        },
                    )
                    .unwrap();
                // ALL_DIGITS переводит индекс оценки обратно в метку Digit.
                ALL_DIGITS[scores_index]
            };

            assert!(!examples.is_empty() && batch_size > 0);
            // Накопление метрик отделено от расчёта итоговых долей.
            let (confusion, loss): ([[usize; 10]; 10], f32) = {
                // confusion[истинная_цифра].features[предсказанная_цифра] считает такие пары.
                // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
                let mut confusion: [[usize; 10]; 10] = [[0usize; 10]; 10];
                let mut loss: f32 = 0.0;
                // Типы переменных: indices: &[usize].
                for indices in examples.chunks(batch_size) {
                    let (input, labels): (Vec<ExampleFeatures>, Vec<Digit>) =
                        batch(digits, indices);
                    let forward_values: ForwardPass = forward(layers, &input);
                    let scores: &Vec<ExampleFeatures> = &forward_values.digit_scores;
                    // cross_entropy возвращает (loss, градиент); для оценки используем только .0.
                    // Функция возвращает среднюю loss одного batch. Умножаем на его размер,
                    // чтобы накопить сумму по картинкам; в конце делим на размер всей выборки.
                    // Так маленький последний batch не получает такой же вес, как большой.
                    loss += cross_entropy(scores, &labels).0 * indices.len() as f32;
                    // Типы переменных: example_values: &ExampleFeatures, label: Digit.
                    for (example_values, label) in scores.iter().zip(labels) {
                        let predicted: Digit = argmax(&example_values.features);
                        confusion[label as usize][predicted as usize] += 1;
                    }
                }
                (confusion, loss)
            };
            (
                loss / examples.len() as f32,
                (0..10)
                    .map(|digits_index: usize| -> usize { confusion[digits_index][digits_index] })
                    .sum::<usize>() as f32
                    / examples.len() as f32,
                confusion,
            )
        };
        // Загружаем все три набора; test используем только после выбора лучших весов.
        let (train_images, validation_images, test): (
            Vec<([NormalizedPixelBrightnessF64; 784], Digit)>,
            Vec<([NormalizedPixelBrightnessF64; 784], Digit)>,
            Vec<([NormalizedPixelBrightnessF64; 784], Digit)>,
        ) = load_mnist_images(&data)?;
        // Из обучения выходят только выбранные веса и необходимые для отчёта значения.
        let (best, best_epoch, batch_size): (Vec<Layer>, usize, usize) = {
            println!(
                "MLP 784 -> 128 -> ReLU -> 10: epochs={epochs}, batch={batch_size}, lr={learning_rate}, seed={seed}, data={}",
                data.display()
            );
            // Для Z = XW+b и входящего G=dL/dZ правило цепочки даёт:
            // dL/dX = G W^T, dL/dW = X^T G, dL/db = сумма G по первому измерению (примерам).
            // Формы: X=[N,D], W=[D,K], G=[N,K]; результаты [N,D], [D,K], [K].
            // Результат — (градиент по входу, градиент весов, градиент смещений).
            let dense_backward =
                |input: &Vec<ExampleFeatures>,
                 weights: &Vec<InputFeatureWeights>,
                 gradient: &Vec<ExampleFeatures>|
                 -> (Vec<ExampleFeatures>, Vec<InputFeatureWeights>, Vec<f32>) {
                    let mut input_gradient: Vec<ExampleFeatures> = vec![
                        ExampleFeatures {
                            features: vec![0.0; weights.len()]
                        };
                        input.len()
                    ];
                    let mut weight_gradient: Vec<InputFeatureWeights> = vec![
                        InputFeatureWeights {
                            output_neurons: vec![0.0; weights[0].output_neurons.len()]
                        };
                        weights.len()
                    ];
                    let mut bias_gradient: Vec<f32> = vec![0.0; weights[0].output_neurons.len()];
                    // Типы переменных: input_index: usize.
                    for input_index in 0..input.len() {
                        // Типы переменных: weights_index: usize.
                        for weights_index in 0..weights.len() {
                            // Типы переменных: output_neurons_index: usize.
                            for output_neurons_index in
                                0..weights[weights_index].output_neurons.len()
                            {
                                input_gradient[input_index].features[weights_index] += gradient
                                    [input_index]
                                    .features[output_neurons_index]
                                    * weights[weights_index].output_neurons[output_neurons_index];
                                weight_gradient[weights_index].output_neurons
                                    [output_neurons_index] += input[input_index].features
                                    [weights_index]
                                    * gradient[input_index].features[output_neurons_index];
                            }
                        }
                        // Типы переменных: biases_index: usize.
                        for biases_index in 0..bias_gradient.len() {
                            bias_gradient[biases_index] +=
                                gradient[input_index].features[biases_index];
                        }
                    }
                    (input_gradient, weight_gradient, bias_gradient)
                };
            // Производная ReLU равна 1 для положительного входа и 0 для отрицательного.
            // В нуле здесь выбираем 0. Положительность сохранённого выхода ReLU даёт эту маску:
            // пропускаем G там, где нейрон был активен, и обнуляем в остальных координатах.
            let relu_backward = |activated: &Vec<ExampleFeatures>,
                                 gradient: &Vec<ExampleFeatures>|
             -> Vec<ExampleFeatures> {
                assert_eq!(activated.len(), gradient.len());
                let mut result: Vec<ExampleFeatures> = gradient.clone();
                // Типы переменных: examples_index: usize.
                for examples_index in 0..activated.len() {
                    assert_eq!(
                        activated[examples_index].features.len(),
                        gradient[examples_index].features.len()
                    );
                    // Типы переменных: features_index: usize.
                    for features_index in 0..activated[examples_index].features.len() {
                        if activated[examples_index].features[features_index] <= 0.0 {
                            result[examples_index].features[features_index] = 0.0;
                        }
                    }
                }
                result
            };

            // Общий SplitMix64 для перемешивания и инициализации весов.
            // Состояние передаём явно: оба потребителя продолжают одну последовательность seed.
            let next_random = |state: &mut u64| -> u64 {
                *state = state.wrapping_add(0x9e3779b97f4a7c15);
                let mut z: u64 = *state;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                z ^ (z >> 31)
            };

            // Fisher–Yates: идём с конца и меняем текущий индекс со случайным индексом 0..=i.
            // Меняется порядок записей train; сами картинки и метки не изменяются.
            let shuffle = |items: &mut [usize], state: &mut u64| -> () {
                // Типы переменных: items_index: usize.
                for items_index in (1..items.len()).rev() {
                    let random_items_index: usize =
                        (next_random(state) % (items_index as u64 + 1)) as usize;
                    items.swap(items_index, random_items_index);
                }
            };
            // Подготовка индексов двух частей и состояния генератора.
            let (training, validation, random_state): (Vec<usize>, Vec<usize>, u64) = {
                // Индексы относятся к своим наборам; перемешиваем только train.
                let training: Vec<usize> = (0..train_images.len()).collect();
                let validation: Vec<usize> = (0..validation_images.len()).collect();
                // Перемешивание возвращает готовые индексы и новое состояние генератора.
                let (training, random_state): (Vec<usize>, u64) = {
                    let mut training: Vec<usize> = training;
                    let mut random_state: u64 = seed;
                    // Первое перемешивание выполняется до инициализации весов.
                    // Затем new_layer продолжит тот же генератор, поэтому порядок вызовов важен.
                    shuffle(&mut training, &mut random_state);

                    (training, random_state)
                };
                // Проверяем, что в обучающей части есть все десять цифр.
                {
                    let mut counts: [usize; 10] = [0usize; 10];
                    // Типы переменных: index: usize.
                    for &index in &training {
                        counts[train_images[index].1 as usize] += 1;
                    }
                    if counts.contains(&0) {
                        return Err("Train должен содержать примеры всех десяти цифр".into());
                    }
                }
                println!(
                    "train={}, validation={}; split=every fifth per class, sorted PNG filenames",
                    training.len(),
                    validation.len()
                );
                (training, validation, random_state)
            };
            let (layers, random_state): (Vec<Layer>, u64) = {
                let mut random_state: u64 = random_state;
                // Создаём W=[input,output] и b=[output]. Смещения начинают с нуля.
                // Веса начинаются с разных малых случайных значений: это помогает скрытым нейронам
                // получать разные градиенты и учить разные признаки.
                let new_layer = |input: usize, output: usize, state: &mut u64| -> Layer {
                    // Равномерная инициализация в [-bound,bound] имеет дисперсию bound²/3=2/input.
                    // Это масштаб He: для ReLU он помогает сохранять разумный размер сигналов в слоях.
                    // Один и тот же способ инициализации здесь применяется ко всем матрицам весов.
                    let bound: f32 = (6.0 / input as f32).sqrt();
                    Layer {
                        weights: {
                            let first_dimension_length: usize = input;
                            let second_dimension_length: usize = output;
                            let mut values: Vec<InputFeatureWeights> = vec![
                                InputFeatureWeights {
                                    output_neurons: vec![0.0; second_dimension_length]
                                };
                                first_dimension_length
                            ];
                            // Типы переменных: first_dimension_index: usize.
                            for first_dimension_index in 0..first_dimension_length {
                                // Типы переменных: second_dimension_index: usize.
                                for second_dimension_index in 0..second_dimension_length {
                                    values[first_dimension_index].output_neurons
                                        [second_dimension_index] = {
                                        // Старшие 24 бита превращаем в f32 в [0,1); 2*uniform-1 даёт [-1,1).
                                        // Умножение на bound задаёт нужный диапазон начальных весов.
                                        let uniform: f32 =
                                            (next_random(state) >> 40) as f32 / 16777216.0;
                                        (2.0 * uniform - 1.0) * bound
                                    };
                                }
                            }
                            values
                        },
                        biases: vec![0.0; output],
                    }
                };
                let layers: Vec<Layer> = vec![
                    // layers[0]: пиксели -> скрытые признаки, W=[784,128], b=[128].
                    new_layer(784, 128, &mut random_state),
                    // layers[1]: скрытые признаки -> классы, W=[128,10], b=[10].
                    new_layer(128, 10, &mut random_state),
                ];
                (layers, random_state)
            };

            let (first_moments, second_moments): (Vec<Layer>, Vec<Layer>) = {
                // Adam помнит историю отдельно для каждого веса и смещения.
                // Создаём два набора нулей той же формы: среднее градиентов m и среднее их квадратов v.
                // Замыкание нужно только для начального создания этих массивов.
                let zero_moments = || -> Vec<Layer> {
                    layers
                        .iter()
                        .map(|layer: &Layer| -> Layer {
                            Layer {
                                weights: vec![
                                    InputFeatureWeights {
                                        output_neurons: vec![0.0; layer.biases.len()]
                                    };
                                    layer.weights.len()
                                ],
                                biases: vec![0.0; layer.biases.len()],
                            }
                        })
                        .collect::<Vec<Layer>>()
                };
                (zero_moments(), zero_moments())
            };
            // Оцениваем ещё не обученную сеть: это точка отсчёта для сравнения.
            let initial_loss: f32 = {
                let initial: (f32, f32, [[usize; 10]; 10]) =
                    evaluate(&layers, &validation_images, &validation, batch_size);

                println!(
                    "epoch=0 validation_loss={:.5} validation_accuracy={:.4}",
                    initial.0, initial.1
                );
                initial.0
            };
            // Всё изменяемое состояние обучения живёт только в этом блоке эпох.
            // За его пределами доступны выбранная модель best и номер её эпохи без mut.
            let (best, best_epoch): (Vec<Layer>, usize) = {
                let mut layers: Vec<Layer> = layers;
                let mut training: Vec<usize> = training;
                let mut random_state: u64 = random_state;
                let mut first_moments: Vec<Layer> = first_moments;
                let mut second_moments: Vec<Layer> = second_moments;
                // Сохраняем отдельную копию лучших весов по validation loss.
                // Копия нужна, потому что следующие эпохи продолжат менять текущие layers.
                // В начале лучший кандидат — ещё не обученная сеть (эпоха 0).
                let mut best: Vec<Layer> = layers.clone();
                let mut best_loss: f32 = initial_loss;
                let mut best_epoch: usize = 0;
                // step — номер ОБНОВЛЕНИЯ весов, не эпохи. Он увеличивается после каждого batch
                // и нужен Adam для поправки на нулевые начальные средние.
                let mut step: i32 = 0i32;
                // Одна эпоха: пройти все batch train, затем оценить validation без обновления весов.
                // Повторяем этот цикл; test до выбора лучшей эпохи не используем.
                // Типы переменных: epoch: usize.
                for epoch in 1..=epochs {
                    // Меняем только порядок обучающих индексов. Генератор продолжает своё состояние,
                    // поэтому каждая эпоха имеет новый, но воспроизводимый при том же seed порядок.
                    shuffle(&mut training, &mut random_state);
                    // Сумма меняется только при обходе train; для отчёта возвращаем готовую train loss.
                    let epoch_loss: f32 = {
                        let mut epoch_loss: f32 = 0.0;
                        // Типы переменных: examples: &[usize].
                        for examples in training.chunks(batch_size) {
                            // Сначала весь прямой и обратный проход по СТАРЫМ весам.
                            // Наружу из этого блока выходят только ошибка batch и градиенты параметров;
                            // активации, окна свёрток и промежуточные производные остаются внутри.
                            let (loss, gradients): (f32, Vec<Layer>) = {
                                let (input, labels): (Vec<ExampleFeatures>, Vec<Digit>) =
                                    batch(&train_images, examples);
                                let forward_values: ForwardPass = forward(&layers, &input);
                                // Получаем среднюю ошибку batch и dL/dscores — начало обратного прохода.
                                // Ошибка должна быть конечной; NaN/∞ означают, что численный расчёт нарушился.
                                let (loss, gradient): (f32, Vec<ExampleFeatures>) =
                                    cross_entropy(&forward_values.digit_scores, &labels);
                                if !loss.is_finite() {
                                    return Err(
                                        "Ошибка обучения не конечна; уменьши learning-rate".into(),
                                    );
                                }
                                // Начинаем с выхода: G по scores даёт ow/ob выходного слоя и dh по hidden.
                                // Веса layers[1] пока не меняем — они ещё нужны для правильного backward.
                                let (dh, ow, ob): (
                                    Vec<ExampleFeatures>,
                                    Vec<InputFeatureWeights>,
                                    Vec<f32>,
                                ) = dense_backward(
                                    &forward_values.hidden_neurons,
                                    &layers[1].weights,
                                    &gradient,
                                );
                                // Перед первым Dense пропускаем dh через маску ReLU hidden.
                                // Затем считаем градиенты hw/hb весов и смещений первого слоя; производные по пикселям не нужны.
                                let (_, hw, hb): (
                                    Vec<ExampleFeatures>,
                                    Vec<InputFeatureWeights>,
                                    Vec<f32>,
                                ) = dense_backward(
                                    &forward_values.input_pixels,
                                    &layers[0].weights,
                                    &relu_backward(&forward_values.hidden_neurons, &dh),
                                );
                                // Градиенты возвращаем в порядке слоёв: сначала скрытый, затем выходной.
                                // Вычисляли их в обратном порядке, но обновление всё равно должно попасть в свои веса.
                                let gradients: Vec<Layer> = vec![
                                    Layer {
                                        weights: hw,
                                        biases: hb,
                                    },
                                    Layer {
                                        weights: ow,
                                        biases: ob,
                                    },
                                ];

                                (loss, gradients)
                            };
                            // Все градиенты рассчитаны по прежним весам; теперь обновляем веса и смещения Adam.
                            {
                                // Теперь все производные уже рассчитаны: можно менять веса.
                                // Обновляем каждый слой по его градиенту, не смешивая новые веса со старым backward.
                                step += 1;
                                // В начале m и v равны нулю, поэтому первые средние занижены.
                                // Деление на 1-β^step исправляет это смещение: β₁=0.9, β₂=0.999.
                                let correction1: f32 = 1.0 - 0.9f32.powi(step);
                                let correction2: f32 = 1.0 - 0.999f32.powi(step);
                                // Типы переменных: layers_index: usize.
                                for layers_index in 0..layers.len() {
                                    // Обновление весов W (поле weights слоя).
                                    // Типы переменных: parameter: &mut f32, m: &mut f32, v: &mut f32, g: f32.
                                    for (((parameter, m), v), &g) in layers[layers_index]
                                        .weights
                                        .iter_mut()
                                        .flat_map(|input_feature: &mut InputFeatureWeights| {
                                            input_feature.output_neurons.iter_mut()
                                        })
                                        .zip(
                                            first_moments[layers_index]
                                                .weights
                                                .iter_mut()
                                                .flat_map(
                                                    |input_feature: &mut InputFeatureWeights| {
                                                        input_feature.output_neurons.iter_mut()
                                                    },
                                                ),
                                        )
                                        .zip(
                                            second_moments[layers_index]
                                                .weights
                                                .iter_mut()
                                                .flat_map(
                                                    |input_feature: &mut InputFeatureWeights| {
                                                        input_feature.output_neurons.iter_mut()
                                                    },
                                                ),
                                        )
                                        .zip(gradients[layers_index].weights.iter().flat_map(
                                            |input_feature: &InputFeatureWeights| {
                                                input_feature.output_neurons.iter()
                                            },
                                        ))
                                    {
                                        // m хранит сглаженный градиент, включая его знак: 90% прежнего + 10% нового.
                                        // v сглаживает квадрат градиента: 99.9% прежнего + 0.1% нового квадрата.
                                        *m = 0.9 * *m + 0.1 * g;
                                        *v = 0.999 * *v + 0.001 * g * g;
                                        // Adam: parameter -= learning_rate * m_hat / (sqrt(v_hat)+epsilon).
                                        // Вычитаем направление градиента, чтобы уменьшать ошибку; большой накопленный
                                        // масштаб градиента уменьшает относительный шаг. epsilon=1e-8 защищает от деления на 0.
                                        *parameter -= learning_rate * (*m / correction1)
                                            / ((*v / correction2).sqrt() + 1e-8);
                                    }
                                    // Та же формула Adam для смещений b (поле biases слоя).
                                    // Типы переменных: parameter: &mut f32, m: &mut f32, v: &mut f32, g: f32.
                                    for (((parameter, m), v), &g) in layers[layers_index]
                                        .biases
                                        .iter_mut()
                                        .zip(first_moments[layers_index].biases.iter_mut())
                                        .zip(second_moments[layers_index].biases.iter_mut())
                                        .zip(gradients[layers_index].biases.iter())
                                    {
                                        *m = 0.9 * *m + 0.1 * g;
                                        *v = 0.999 * *v + 0.001 * g * g;

                                        *parameter -= learning_rate * (*m / correction1)
                                            / ((*v / correction2).sqrt() + 1e-8);
                                    }
                                }
                            }
                            // Накопленная train loss относится к моментам обновлений: разные batch оценивались
                            // при разных весах. Умножение на N даёт сумму ошибок, чтобы правильно усреднить эпоху.
                            epoch_loss += loss * examples.len() as f32;
                        }
                        epoch_loss
                    };
                    // Validation — отложенные картинки: они помогают выбрать эпоху, но не меняют веса.
                    // Падающая train loss при растущей validation loss может указывать на переобучение.
                    let metrics: (f32, f32, [[usize; 10]; 10]) =
                        evaluate(&layers, &validation_images, &validation, batch_size);
                    if !metrics.0.is_finite() {
                        return Err("Validation loss не конечна".into());
                    }
                    // Запоминаем эпоху только при уменьшении validation loss (metrics.0).
                    // Выбор идёт по ошибке, а не по accuracy: уверенные неправильные ответы тоже важны.
                    // Последняя эпоха не обязана быть лучшей, поэтому на test пойдут сохранённые best.
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

                (best, best_epoch)
            };
            (best, best_epoch, batch_size)
        };
        // Обучение завершено: оцениваем сохранённую модель best на официальном test.
        // В этом блоке нет индексов train и состояния Adam; обновления весов не выполняются.
        {
            let test_indices: Vec<usize> = (0..test.len()).collect();
            // Итоговый отчёт на официальном test: используем выбранную по validation копию best.
            // По test не выбираем веса, число эпох или скорость обучения.
            let metrics: (f32, f32, [[usize; 10]; 10]) =
                evaluate(&best, &test, &test_indices, batch_size);
            println!(
                "selected_epoch={best_epoch}; test={} loss={:.5} accuracy={:.4} elapsed={:.1}s",
                test.len(),
                metrics.0,
                metrics.1,
                start.elapsed().as_secs_f32()
            );
            println!("Ошибки[истинная цифра][прогноз]: оба измерения соответствуют цифрам 0..9");
            // Типы переменных: example_values: [usize; 10].
            for example_values in metrics.2 {
                println!("{example_values:?}");
            }
        }
    }
    Ok(())
}
