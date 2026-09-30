// Урок 21.6. Практика: обучение нескольких взвешенных слоёв с нелинейным преобразованием выходов.
// Связь с принятой терминологией: Многослойный перцептрон со слоями, активациями и мини пакетами.
// Зачем здесь эта тема: Многослойная сеть соединяет матричные веса, bias, активации и обучение мини
//   пакетами.
// Почему код устроен так: Используем небольшие слои, чтобы проследить размеры и отдельные
//   обновления.
// Представь: Вход проходит несколько слоёв, а обновление весов происходит после расчёта ошибки и
//   градиентов группы.
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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Учебные реализации математических операций для этого урока.");

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::NEG_INFINITY || value < -745.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.0;
        }
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::INFINITY || value > 709.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return f64::INFINITY;
        }
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value < 0.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        trace_note!("Создаём изменяемое значение `reduced` для следующих операций.");
        let mut reduced: f64 = value;
        trace_step!(reduced);
        trace_note!("Инициализируем изменяемый накопитель `halving_count` начальным состоянием.");
        let mut halving_count: i32 = 0;
        trace_step!(halving_count);
        trace_note!(
            "Уменьшаем аргумент до ≤0.5: на таком интервале ряд Тейлора для exp сходится быстро."
        );
        while reduced > 0.5 {
            trace_note!("Масштабируем текущую величину делением.");
            reduced /= 2.0;
            trace_step!(reduced);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            halving_count += 1;
            trace_step!(halving_count);
        }
        trace_note!("Создаём изменяемое значение `term` для следующих операций.");
        let mut term: f64 = 1.0;
        trace_step!(term);
        trace_note!("Создаём изменяемое значение `result` для следующих операций.");
        let mut result: f64 = 1.0;
        trace_step!(result);
        trace_note!(
            "Берём 30 членов ряда exp(y)=Σ y^k/k!; это предел приближения для учебных входов."
        );
        for term_index in 1..=30 {
            trace_step!(term_index);
            trace_note!("Умножаем накопленное значение на очередной множитель.");
            term *= reduced / term_index as f64;
            trace_step!(term);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            result += term;
            trace_step!(result);
        }
        trace_note!(
            "Восстанавливаем exp(value): каждое возведение в квадрат отменяет одно деление аргумента на 2."
        );
        for _ in 0..halving_count {
            trace_note!("Умножаем накопленное значение на очередной множитель.");
            result *= result;
            trace_step!(result);
        }
        trace_note!("Используем ранее рассчитанное значение `result` в текущем выражении.");
        result
    }

    trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    const XOR: [([f64; 2], f64); 4] = [
        ([0., 0.], 0.),
        ([0., 1.], 1.),
        ([1., 0.], 1.),
        ([1., 1.], 0.),
    ];

    trace_note!(
        "Объявляем повторно используемое вычисление `calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score`; параметры ниже задают его входы."
    );
    trace_note!("Оценку модели до преобразования в вероятность называют logit.");
    /// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
    fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
        raw_model_score: f64,
    ) -> f64 {
        trace_note!("Делим значения, получая нормированную величину или среднее.");
        1. / (1. + approximate_e_to_power_by_summing_power_over_factorial_terms(-raw_model_score))
    }
    trace_note!(
        "d calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(z)/dz = calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(z) * (1 − calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(z))."
    );
    /// Производная сигмоиды по её входу: output·(1−output), если output — уже вычисленная сигмоида.
    fn calculate_sigmoid_slope_by_multiplying_output_by_one_minus_output(
        sigmoid_output: f64,
    ) -> f64 {
        trace_note!("Умножаем величины согласно используемой формуле.");
        sigmoid_output * (1.0 - sigmoid_output)
    }

    trace_note!(
        "Описываем тип `NeuralNetwork`, чтобы явно хранить состояние и допустимые варианты."
    );
    trace_note!("`hidden_weights` задаёт соответствующее входное значение или поле структуры.");
    trace_note!("`output_weights` задаёт соответствующее входное значение или поле структуры.");
    #[derive(Debug)]
    struct NeuralNetwork {
        hidden_weights: [[f64; 3]; 2],

        output_weights: [f64; 3],
    }

    trace_note!("Шаг: Обучаем сеть с двумя скрытыми нейронами на таблице XOR.");
    let network: NeuralNetwork = (|| -> NeuralNetwork {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Обучаем сеть на всех комбинациях XOR повторяющимися шагами градиента.");
        trace_note!("Начальные веса задают воспроизводимый старт обучения.");
        trace_note!("Задаём начальные веса двух нейронов скрытого слоя.");
        trace_note!("Задаём начальные веса выходного нейрона и его смещение.");
        let mut network: NeuralNetwork = NeuralNetwork {
            hidden_weights: [[0.8, -0.5, 0.2], [-0.3, 0.9, -0.1]],

            output_weights: [0.7, -0.8, 0.1],
        };
        trace_step!(network);
        trace_note!(
            "XOR требует многократных обновлений весов: 20 000 эпох выбраны для сходимости примера."
        );
        trace_note!("В каждой эпохе ниже обрабатываются все четыре учебных случая.");
        for epoch in 0..20_000 {
            trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for &(features, expected_output) in &XOR {
                trace_step!(features);
                trace_step!(expected_output);
                trace_note!(
                    "0.5 — скорость обучения для этого маленького XOR: вес меняется на половину градиента."
                );
                let learning_rate: f64 = 0.5;
                trace_step!(learning_rate);
                trace_note!(
                    "Прямой проход: сначала два скрытых нейрона, затем выходная вероятность."
                );
                trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
                trace_note!("Обновляем состояние объекта результатом текущей операции.");
                trace_note!("Умножаем величины согласно используемой формуле.");
                trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
                trace_note!("Обновляем состояние объекта результатом текущей операции.");
                trace_note!("Умножаем величины согласно используемой формуле.");
                trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                let hidden_outputs: [f64; 2] = [
                    calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                        network.hidden_weights[0][0] * features[0]
                            + network.hidden_weights[0][1] * features[1]
                            + network.hidden_weights[0][2],
                    ),
                    calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                        network.hidden_weights[1][0] * features[0]
                            + network.hidden_weights[1][1] * features[1]
                            + network.hidden_weights[1][2],
                    ),
                ];
                trace_step!(hidden_outputs);
                trace_note!(
                    "Сохраняем рассчитанное значение `output_probability` для следующих операций."
                );
                trace_note!("Обновляем состояние объекта результатом текущей операции.");
                trace_note!("Умножаем величины согласно используемой формуле.");
                trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                let output_probability: f64 =
                    calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                        network.output_weights[0] * hidden_outputs[0]
                            + network.output_weights[1] * hidden_outputs[1]
                            + network.output_weights[2],
                    );
                trace_step!(output_probability);
                trace_note!(
                    "Правило цепочки даёт градиент ошибки для выхода и каждого скрытого нейрона."
                );
                trace_note!(
                    "Производную функции по параметру или вектор таких производных называют gradient."
                );
                trace_note!(
                    "Используем ранее рассчитанное значение `output_probability` в текущем выражении."
                );
                let output_loss_rate_of_change: f64 = (|| -> f64 {
                    trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    trace_note!(
                        "Для квадратичной ошибки 1/2*(prediction−target)² производная по prediction — разность."
                    );
                    trace_note!("Сохраняем результат этого шага в `prediction`.");
                    let prediction: f64 = output_probability;
                    trace_step!(prediction);
                    trace_note!("Сохраняем рассчитанное значение `target` для следующих операций.");
                    let target: f64 = expected_output;
                    trace_step!(target);
                    trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                    trace_note!("Вычисляем значение по указанной формуле.");
                    prediction - target
                })()
                    * calculate_sigmoid_slope_by_multiplying_output_by_one_minus_output(
                        output_probability,
                    );
                trace_step!(output_loss_rate_of_change);
                trace_note!(
                    "Создаём набор значений `hidden_layer_loss_rates_of_change` для следующего шага примера."
                );
                trace_note!(
                    "Используем ранее рассчитанное значение `output_loss_rate_of_change` в текущем выражении."
                );
                trace_note!("Добавляем этот член в составное арифметическое выражение.");
                trace_note!("Добавляем этот член в составное арифметическое выражение.");
                trace_note!(
                    "Используем ранее рассчитанное значение `output_loss_rate_of_change` в текущем выражении."
                );
                trace_note!("Добавляем этот член в составное арифметическое выражение.");
                trace_note!("Добавляем этот член в составное арифметическое выражение.");
                let hidden_layer_loss_rates_of_change: [f64; 2] = [
                    output_loss_rate_of_change
                        * network.output_weights[0]
                        * calculate_sigmoid_slope_by_multiplying_output_by_one_minus_output(
                            hidden_outputs[0],
                        ),
                    output_loss_rate_of_change
                        * network.output_weights[1]
                        * calculate_sigmoid_slope_by_multiplying_output_by_one_minus_output(
                            hidden_outputs[1],
                        ),
                ];
                trace_step!(hidden_layer_loss_rates_of_change);
                trace_note!(
                    "Обновляем связи и смещения; последний столбец каждой матрицы хранит bias."
                );
                for hidden_neuron_index in 0..2 {
                    trace_step!(hidden_neuron_index);
                    trace_note!("Обновляем параметр модели с учётом вычисленного градиента.");
                    trace_note!("Умножаем величины согласно используемой формуле.");
                    network.output_weights[hidden_neuron_index] -= learning_rate
                        * output_loss_rate_of_change
                        * hidden_outputs[hidden_neuron_index];
                    trace_step!(network);
                    trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for feature_index in 0..2 {
                        trace_step!(feature_index);
                        trace_note!("Обновляем параметр модели с учётом вычисленного градиента.");
                        trace_note!("Добавляем этот член в составное арифметическое выражение.");
                        trace_note!("Добавляем этот член в составное арифметическое выражение.");
                        network.hidden_weights[hidden_neuron_index][feature_index] -= learning_rate
                            * hidden_layer_loss_rates_of_change[hidden_neuron_index]
                            * features[feature_index];
                        trace_step!(network);
                    }
                    trace_note!("Обновляем параметр модели с учётом вычисленного градиента.");
                    trace_note!("Умножаем величины согласно используемой формуле.");
                    network.hidden_weights[hidden_neuron_index][2] -=
                        learning_rate * hidden_layer_loss_rates_of_change[hidden_neuron_index];
                    trace_step!(network);
                }
                trace_note!("Обновляем параметр модели с учётом вычисленного градиента.");
                network.output_weights[2] -= learning_rate * output_loss_rate_of_change;
                trace_step!(network);
            }
            if matches!(epoch, 0 | 1 | 9 | 99 | 999 | 9_999 | 19_999) {
                println!("после эпохи {}: веса сети={network:?}", epoch + 1);
            }
        }
        trace_note!("Используем ранее рассчитанное значение `network` в текущем выражении.");
        network
    })();
    trace_step!(network);
    trace_note!("Шаг: Проверяем все четыре комбинации входных битов.");
    for (features, expected_output) in XOR {
        trace_step!(features);
        trace_step!(expected_output);
        trace_note!("Считаем выход двух скрытых нейронов и итоговую вероятность.");
        let output_probability: f64 = {
            trace_note!("Создаём набор значений `hidden_outputs` для следующего шага примера.");
            trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
            trace_note!("Обновляем состояние объекта результатом текущей операции.");
            trace_note!("Умножаем величины согласно используемой формуле.");
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
            trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
            trace_note!("Обновляем состояние объекта результатом текущей операции.");
            trace_note!("Умножаем величины согласно используемой формуле.");
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
            let hidden_outputs: [f64; 2] = [
                calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                    network.hidden_weights[0][0] * features[0]
                        + network.hidden_weights[0][1] * features[1]
                        + network.hidden_weights[0][2],
                ),
                calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                    network.hidden_weights[1][0] * features[0]
                        + network.hidden_weights[1][1] * features[1]
                        + network.hidden_weights[1][2],
                ),
            ];
            trace_step!(hidden_outputs);
            trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
            trace_note!("Обновляем состояние объекта результатом текущей операции.");
            trace_note!("Умножаем величины согласно используемой формуле.");
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
            calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                network.output_weights[0] * hidden_outputs[0]
                    + network.output_weights[1] * hidden_outputs[1]
                    + network.output_weights[2],
            )
        };
        trace_step!(output_probability);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
        trace_note!(
            "Используем ранее рассчитанное значение `output_probability` в текущем выражении."
        );
        println!(
            "{features:?} -> {:.3} (expected {expected_output})",
            output_probability
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_two_feature_training_points_by_class();
}

// Строим график по результатам урока.
fn plot_two_feature_training_points_by_class() {
    trace_note!("Значения из этого урока на графике.");
    let class_zero_points: Vec<(f64, f64)> = [(0.0, 0.0), (1.0, 1.0)].to_vec();
    trace_note!("Собираем значения для `class_one_points` в коллекцию.");
    let class_one_points: Vec<(f64, f64)> = [(0.0, 1.0), (1.0, 0.0)].to_vec();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "XOR: обучающие примеры",
        "первый признак",
        "второй признак",
        &[
            lesson_visualization::Series {
                name: "класс 0",

                points: &class_zero_points,
            },
            lesson_visualization::Series {
                name: "класс 1",

                points: &class_one_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
