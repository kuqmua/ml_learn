// Сводная практика 21. Многослойный перцептрон со слоями, активациями и мини пакетами.
//
// Что повторяем вместе: слои, параметры, активации, инициализация, mini-batch.
// Зачем это нужно: Многослойная сеть создаёт нелинейный прогноз; задача XOR показывает, зачем нужен скрытый
//   слой.
// Что показывает программа: Обучаем сеть с двумя скрытыми нейронами на таблице XOR. Проверяем все четыре
//   комбинации входных битов.
// Что проверить при изменении примера: Проверь все четыре комбинации XOR; сохрани кривую loss и зафиксируй
//   seed.
// Дополнительная практика: Обучи MLP на XOR с ручным backprop или графом предыдущего урока.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Учебные реализации математических операций для этого урока.

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    fn approximate_exponential_with_taylor_series(value: f64) -> f64 {
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == f64::NEG_INFINITY || value < -745.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 0.0;
        }
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == f64::INFINITY || value > 709.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return f64::INFINITY;
        }
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value < 0.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 1.0 / approximate_exponential_with_taylor_series(-value);
        }
        // Создаём изменяемое значение `reduced` для следующих операций.
        let mut reduced: f64 = value;
        lesson_trace::trace_step!(reduced);
        // Инициализируем изменяемый накопитель `halving_count` начальным состоянием.
        let mut halving_count: i32 = 0;
        lesson_trace::trace_step!(halving_count);
        // Уменьшаем аргумент до ≤0.5: на таком интервале ряд Тейлора для exp сходится быстро.
        while reduced > 0.5 {
            // Масштабируем текущую величину делением.
            reduced /= 2.0;
            lesson_trace::trace_step!(reduced);
            // Прибавляем очередной вклад к ранее накопленному результату.
            halving_count += 1;
            lesson_trace::trace_step!(halving_count);
        }
        // Создаём изменяемое значение `term` для следующих операций.
        let mut term: f64 = 1.0;
        lesson_trace::trace_step!(term);
        // Создаём изменяемое значение `result` для следующих операций.
        let mut result: f64 = 1.0;
        lesson_trace::trace_step!(result);
        // Берём 30 членов ряда exp(y)=Σ y^k/k!; это предел приближения для учебных входов.
        for term_index in 1..=30 {
            lesson_trace::trace_step!(term_index);
            // Умножаем накопленное значение на очередной множитель.
            term *= reduced / term_index as f64;
            lesson_trace::trace_step!(term);
            // Прибавляем очередной вклад к ранее накопленному результату.
            result += term;
            lesson_trace::trace_step!(result);
        }
        // Восстанавливаем exp(value): каждое возведение в квадрат отменяет одно деление аргумента на 2.
        for _ in 0..halving_count {
            // Умножаем накопленное значение на очередной множитель.
            result *= result;
            lesson_trace::trace_step!(result);
        }
        // Используем ранее рассчитанное значение `result` в текущем выражении.
        result
    }

    // Фиксируем демонстрационные данные на время выполнения программы.
    const XOR: [([f64; 2], f64); 4] = [
        // Составляем результат из вычисленных значений в указанном порядке.
        ([0., 0.], 0.),
        // Составляем результат из вычисленных значений в указанном порядке.
        ([0., 1.], 1.),
        // Составляем результат из вычисленных значений в указанном порядке.
        ([1., 0.], 1.),
        // Составляем результат из вычисленных значений в указанном порядке.
        ([1., 1.], 0.),
    ];

    // Объявляем повторно используемое вычисление `convert_logit_to_probability`; параметры ниже задают его входы.
    // Оценку модели до преобразования в вероятность называют logit.
    fn convert_logit_to_probability(raw_model_score: f64) -> f64 {
        // Делим значения, получая нормированную величину или среднее.
        1. / (1. + approximate_exponential_with_taylor_series(-raw_model_score))
    }
    // d sigmoid_activation_of_raw_score(z)/dz = sigmoid_activation_of_raw_score(z) * (1 − sigmoid_activation_of_raw_score(z)).
    fn calculate_sigmoid_derivative_from_output(sigmoid_output: f64) -> f64 {
        // Умножаем величины согласно используемой формуле.
        sigmoid_output * (1.0 - sigmoid_output)
    }

    // Описываем тип `NeuralNetwork`, чтобы явно хранить состояние и допустимые варианты.
    #[derive(Debug)]
    struct NeuralNetwork {
        // `hidden_weights` задаёт соответствующее входное значение или поле структуры.
        hidden_weights: [[f64; 3]; 2],
        // `output_weights` задаёт соответствующее входное значение или поле структуры.
        output_weights: [f64; 3],
    }

    // Шаг: Обучаем сеть с двумя скрытыми нейронами на таблице XOR.
    let network: NeuralNetwork = (|| -> NeuralNetwork {
        // Используем подготовленное значение в следующем шаге примера.
        /* Обучаем сеть на всех комбинациях XOR повторяющимися шагами градиента. */
        // Начальные веса задают воспроизводимый старт обучения.
        let mut network: NeuralNetwork = NeuralNetwork {
            // Задаём начальные веса двух нейронов скрытого слоя.
            hidden_weights: [[0.8, -0.5, 0.2], [-0.3, 0.9, -0.1]],
            // Задаём начальные веса выходного нейрона и его смещение.
            output_weights: [0.7, -0.8, 0.1],
        };
        lesson_trace::trace_step!(network);
        // XOR требует многократных обновлений весов: 20 000 эпох выбраны для сходимости примера.
        // В каждой эпохе ниже обрабатываются все четыре учебных случая.
        for epoch in 0..20_000 {
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for &(features, expected_output) in &XOR {
                lesson_trace::trace_step!(features);
                lesson_trace::trace_step!(expected_output);
                // 0.5 — скорость обучения для этого маленького XOR: вес меняется на половину градиента.
                let learning_rate: f64 = 0.5;
                lesson_trace::trace_step!(learning_rate);
                // Прямой проход: сначала два скрытых нейрона, затем выходная вероятность.
                let hidden_outputs: [f64; 2] = [
                    // Вызываем нужное вычисление с подготовленными аргументами.
                    convert_logit_to_probability(
                        // Обновляем состояние объекта результатом текущей операции.
                        network.hidden_weights[0][0] * features[0]
                            // Умножаем величины согласно используемой формуле.
                            + network.hidden_weights[0][1] * features[1]
                            // Складываем или вычитаем величины согласно используемой формуле.
                            + network.hidden_weights[0][2],
                    ),
                    // Вызываем нужное вычисление с подготовленными аргументами.
                    convert_logit_to_probability(
                        // Обновляем состояние объекта результатом текущей операции.
                        network.hidden_weights[1][0] * features[0]
                            // Умножаем величины согласно используемой формуле.
                            + network.hidden_weights[1][1] * features[1]
                            // Складываем или вычитаем величины согласно используемой формуле.
                            + network.hidden_weights[1][2],
                    ),
                ];
                lesson_trace::trace_step!(hidden_outputs);
                // Сохраняем рассчитанное значение `output_probability` для следующих операций.
                let output_probability: f64 = convert_logit_to_probability(
                    // Обновляем состояние объекта результатом текущей операции.
                    network.output_weights[0] * hidden_outputs[0]
                        // Умножаем величины согласно используемой формуле.
                        + network.output_weights[1] * hidden_outputs[1]
                        // Складываем или вычитаем величины согласно используемой формуле.
                        + network.output_weights[2],
                );
                lesson_trace::trace_step!(output_probability);
                // Правило цепочки даёт градиент ошибки для выхода и каждого скрытого нейрона.
                // Производную функции по параметру или вектор таких производных называют gradient.
                let output_loss_rate_of_change: f64 = (|| -> f64 {
                    // Используем подготовленное значение в следующем шаге примера.
                    /* Для квадратичной ошибки 1/2*(prediction−target)² производная по prediction — разность. */
                    // Сохраняем результат этого шага в `prediction`.
                    let prediction: f64 = output_probability;
                    lesson_trace::trace_step!(prediction);
                    // Сохраняем рассчитанное значение `target` для следующих операций.
                    let target: f64 = expected_output;
                    lesson_trace::trace_step!(target);
                    // Складываем или вычитаем величины согласно используемой формуле.
                    prediction - target
                    // Вычисляем значение по указанной формуле.
                })()
                    * calculate_sigmoid_derivative_from_output(
                        // Используем ранее рассчитанное значение `output_probability` в текущем выражении.
                        output_probability,
                    );
                lesson_trace::trace_step!(output_loss_rate_of_change);
                // Создаём набор значений `hidden_layer_loss_rates_of_change` для следующего шага примера.
                let hidden_layer_loss_rates_of_change: [f64; 2] = [
                    // Используем ранее рассчитанное значение `output_loss_rate_of_change` в текущем выражении.
                    output_loss_rate_of_change
                        // Добавляем этот член в составное арифметическое выражение.
                        * network.output_weights[0]
                        // Добавляем этот член в составное арифметическое выражение.
                        * calculate_sigmoid_derivative_from_output(hidden_outputs[0]),
                    // Используем ранее рассчитанное значение `output_loss_rate_of_change` в текущем выражении.
                    output_loss_rate_of_change
                        // Добавляем этот член в составное арифметическое выражение.
                        * network.output_weights[1]
                        // Добавляем этот член в составное арифметическое выражение.
                        * calculate_sigmoid_derivative_from_output(hidden_outputs[1]),
                ];
                lesson_trace::trace_step!(hidden_layer_loss_rates_of_change);
                // Обновляем связи и смещения; последний столбец каждой матрицы хранит bias.
                for hidden_neuron_index in 0..2 {
                    lesson_trace::trace_step!(hidden_neuron_index);
                    // Обновляем параметр модели с учётом вычисленного градиента.
                    network.output_weights[hidden_neuron_index] -=
                        // Умножаем величины согласно используемой формуле.
                        learning_rate * output_loss_rate_of_change * hidden_outputs[hidden_neuron_index];
                    lesson_trace::trace_step!(network);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for feature_index in 0..2 {
                        lesson_trace::trace_step!(feature_index);
                        // Обновляем параметр модели с учётом вычисленного градиента.
                        network.hidden_weights[hidden_neuron_index][feature_index] -= learning_rate
                            // Добавляем этот член в составное арифметическое выражение.
                            * hidden_layer_loss_rates_of_change[hidden_neuron_index]
                            // Добавляем этот член в составное арифметическое выражение.
                            * features[feature_index];
                        lesson_trace::trace_step!(network);
                    }
                    // Обновляем параметр модели с учётом вычисленного градиента.
                    network.hidden_weights[hidden_neuron_index][2] -=
                        // Умножаем величины согласно используемой формуле.
                        learning_rate * hidden_layer_loss_rates_of_change[hidden_neuron_index];
                    lesson_trace::trace_step!(network);
                }
                // Обновляем параметр модели с учётом вычисленного градиента.
                network.output_weights[2] -= learning_rate * output_loss_rate_of_change;
                lesson_trace::trace_step!(network);
            }
            if matches!(epoch, 0 | 1 | 9 | 99 | 999 | 9_999 | 19_999) {
                println!("после эпохи {}: веса сети={network:?}", epoch + 1);
            }
        }
        // Используем ранее рассчитанное значение `network` в текущем выражении.
        network
    })();
    lesson_trace::trace_step!(network);
    // Шаг: Проверяем все четыре комбинации входных битов.
    for (features, expected_output) in XOR {
        lesson_trace::trace_step!(features);
        lesson_trace::trace_step!(expected_output);
        // Считаем выход двух скрытых нейронов и итоговую вероятность.
        let output_probability: f64 = {
            // Создаём набор значений `hidden_outputs` для следующего шага примера.
            let hidden_outputs: [f64; 2] = [
                // Вызываем нужное вычисление с подготовленными аргументами.
                convert_logit_to_probability(
                    // Обновляем состояние объекта результатом текущей операции.
                    network.hidden_weights[0][0] * features[0]
                        // Умножаем величины согласно используемой формуле.
                        + network.hidden_weights[0][1] * features[1]
                        // Складываем или вычитаем величины согласно используемой формуле.
                        + network.hidden_weights[0][2],
                ),
                // Вызываем нужное вычисление с подготовленными аргументами.
                convert_logit_to_probability(
                    // Обновляем состояние объекта результатом текущей операции.
                    network.hidden_weights[1][0] * features[0]
                        // Умножаем величины согласно используемой формуле.
                        + network.hidden_weights[1][1] * features[1]
                        // Складываем или вычитаем величины согласно используемой формуле.
                        + network.hidden_weights[1][2],
                ),
            ];
            lesson_trace::trace_step!(hidden_outputs);
            // Вызываем нужное вычисление с подготовленными аргументами.
            convert_logit_to_probability(
                // Обновляем состояние объекта результатом текущей операции.
                network.output_weights[0] * hidden_outputs[0]
                    // Умножаем величины согласно используемой формуле.
                    + network.output_weights[1] * hidden_outputs[1]
                    // Складываем или вычитаем величины согласно используемой формуле.
                    + network.output_weights[2],
            )
        };
        lesson_trace::trace_step!(output_probability);
        // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
        println!(
            // Складываем или вычитаем величины согласно используемой формуле.
            "{features:?} -> {:.3} (expected {expected_output})",
            // Используем ранее рассчитанное значение `output_probability` в текущем выражении.
            output_probability
        );
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_multilayer_perceptron_with_layers_activations_and_mini_batches();
}

// Строим график по результатам урока.
fn visualize_practice_multilayer_perceptron_with_layers_activations_and_mini_batches() {
    // Значения из этого урока на графике.
    let class_zero_points: Vec<(f64, f64)> = [(0.0, 0.0), (1.0, 1.0)].to_vec();
    // Собираем значения для `class_one_points` в коллекцию.
    let class_one_points: Vec<(f64, f64)> = [(0.0, 1.0), (1.0, 0.0)].to_vec();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "XOR: обучающие примеры",
        // Указываем подпись горизонтальной оси.
        "первый признак",
        // Указываем подпись вертикальной оси.
        "второй признак",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "класс 0",
                // Передаём рассчитанные координаты точек.
                points: &class_zero_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "класс 1",
                // Передаём рассчитанные координаты точек.
                points: &class_one_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
