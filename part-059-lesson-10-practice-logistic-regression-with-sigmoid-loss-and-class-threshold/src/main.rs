// Сводная практика 10. Логистическая регрессия, сигмоида, ошибка и порог.
// Почему этот урок сейчас: Классификация требует согласовать логит, вероятность, ошибку обучения и порог решения.
// Почему пример устроен так: Проводим одни и те же примеры через все четыре шага и проверяем итоговый класс.
//
// Что повторяем вместе: сигмоида, log-loss, вероятности, порог классификации.
// Зачем это нужно: Логистическая регрессия превращает линейную оценку в вероятность класса и обучается по
//   логарифмической ошибке.
// Что показывает программа: Измеряем loss модели с нулевыми коэффициентами. Подбираем параметры градиентным
//   спуском. Сравниваем loss до и после обучения и выводим вероятность класса.
// Что проверить при изменении примера: Проверь вероятности в [0,1], улучшение loss и влияние порога на
//   precision/recall.
// Дополнительная практика: Реализуй бинарный классификатор и стабильный расчёт log-loss без log(0).

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
    const TRAINING_EXAMPLES: [(f64, f64); 6] = [
        // Учебная пара: вход -3., ожидаемое значение 0..
        (-3., 0.),
        // Учебная пара: вход -2., ожидаемое значение 0..
        (-2., 0.),
        // Учебная пара: вход -1., ожидаемое значение 0..
        (-1., 0.),
        // Учебная пара: вход 1., ожидаемое значение 1..
        (1., 1.),
        // Учебная пара: вход 2., ожидаемое значение 1..
        (2., 1.),
        // Учебная пара: вход 3., ожидаемое значение 1..
        (3., 1.),
    ];

    // Объявляем повторно используемое вычисление `calculate_binary_cross_entropy_from_logits`; параметры ниже задают его входы.
    fn calculate_binary_cross_entropy_from_logits(
        // Получаем набор наблюдений, по которому считаем ошибку или градиент.
        data: &[(f64, f64)],
        // Параметр `weight` передаёт коэффициент при признаке.
        weight: f64,
        // Параметр `bias` передаёт свободный член модели.
        bias: f64,
        // Указываем тип возвращаемого значения.
    ) -> f64 {
        // Инициализируем изменяемый накопитель `loss_sum` начальным состоянием.
        let mut loss_sum: f64 = 0.0;
        lesson_trace::trace_step!(loss_sum);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for &(feature_value, target_label) in data {
            lesson_trace::trace_step!(feature_value);
            lesson_trace::trace_step!(target_label);
            // Умножаем значения и сохраняем результат в `raw_model_score`.
            // Оценку модели до преобразования в вероятность называют logit.
            let raw_model_score: f64 = weight * feature_value + bias;
            lesson_trace::trace_step!(raw_model_score);
            // Прибавляем очередной вклад к ранее накопленному результату.
            loss_sum += (|| -> f64 {
                // Используем подготовленное значение в следующем шаге примера.
                /* Выбираем большее из двух чисел для формул softmax, log-loss и Q-learning. */
                // Сохраняем результат этого шага в `first`.
                let first: f64 = raw_model_score;
                lesson_trace::trace_step!(first);
                // Инициализируем значение `second` начальным состоянием.
                let second: f64 = 0.;
                lesson_trace::trace_step!(second);
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if first > second { first } else { second }
            // Вычисляем значение по указанной формуле.
            })() - target_label * raw_model_score
                // Складываем или вычитаем величины согласно используемой формуле.
                + (|| -> f64 {
                    // Обновляем значение результатом текущего вычисления.
                    /* ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1). */
                    // Сохраняем результат этого шага в `value`.
                    let value: f64 = 1.
                        // Складываем или вычитаем величины согласно используемой формуле.
                        + approximate_exponential_with_taylor_series(-(|| -> f64 {
                            // Используем подготовленное значение в следующем шаге примера.
                            /* Модуль числа по определению: меняем знак только у отрицательного числа. */
                            // Сохраняем результат этого шага в `value`.
                            let value: f64 = raw_model_score;
                            lesson_trace::trace_step!(value);
                            // Проверяем условие и выбираем соответствующую ветку алгоритма.
                            if value < 0.0 { -value } else { value }
                        })());
                    lesson_trace::trace_step!(value);
                    // Проверяем обязательное условие до дальнейшего вычисления.
                    assert!(
                        // Передаём очередное значение в составе результата или вызова.
                        value > 0.0,
                        // Подставляем результаты в этот шаблон вывода или текстового значения.
                        "логарифм определён только для положительных чисел"
                    );
                    // Проверяем условие и выбираем соответствующую ветку алгоритма.
                    if value == f64::INFINITY {
                        // Завершаем текущий расчёт и возвращаем найденное значение.
                        return f64::INFINITY;
                    }
                    // Создаём изменяемое значение `scaled` для следующих операций.
                    let mut scaled: f64 = value;
                    lesson_trace::trace_step!(scaled);
                    // Инициализируем изменяемый накопитель `power_of_two` начальным состоянием.
                    let mut power_of_two: i32 = 0i32;
                    lesson_trace::trace_step!(power_of_two);
                    // Повторяем вычисление, пока выполняется указанное условие.
                    while scaled >= 2.0 {
                        // Масштабируем текущую величину делением.
                        scaled /= 2.0;
                        lesson_trace::trace_step!(scaled);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        power_of_two += 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    // Повторяем вычисление, пока выполняется указанное условие.
                    while scaled < 1.0 {
                        // Умножаем накопленное значение на очередной множитель.
                        scaled *= 2.0;
                        lesson_trace::trace_step!(scaled);
                        // Вычитаем очередной вклад из текущего значения параметра.
                        power_of_two -= 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    // Этот ряд — учебное раскрытие `value.ln()`; он может работать медленнее и отличаться по точности.
                    // Объявляем повторно используемое вычисление `sum_logarithm_series_terms`; параметры ниже задают его входы.
                    fn sum_logarithm_series_terms(value: f64) -> f64 {
                        // Нормируем или усредняем величину делением и сохраняем её в `ratio`.
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        lesson_trace::trace_step!(ratio);
                        // Умножаем значения и сохраняем результат в `ratio_squared`.
                        let ratio_squared: f64 = ratio * ratio;
                        lesson_trace::trace_step!(ratio_squared);
                        // Создаём изменяемое значение `term` для следующих операций.
                        let mut term: f64 = ratio;
                        lesson_trace::trace_step!(term);
                        // Инициализируем изменяемый накопитель `result` начальным состоянием.
                        let mut result: f64 = 0.0;
                        lesson_trace::trace_step!(result);
                        // Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1).
                        // Это конечное приближение: для положительного value выполняется |ratio| < 1.
                        for term_index in 0..40 {
                            lesson_trace::trace_step!(term_index);
                            // Прибавляем очередной вклад к ранее накопленному результату.
                            result += term / (2 * term_index + 1) as f64;
                            lesson_trace::trace_step!(result);
                            // Умножаем накопленное значение на очередной множитель.
                            term *= ratio_squared;
                            lesson_trace::trace_step!(term);
                        }
                        // Умножаем величины согласно используемой формуле.
                        2.0 * result
                    }
                    // Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций.
                    let logarithm_of_two: f64 = sum_logarithm_series_terms(2.0);
                    lesson_trace::trace_step!(logarithm_of_two);
                    // Умножаем величины согласно используемой формуле.
                    sum_logarithm_series_terms(scaled) + power_of_two as f64 * logarithm_of_two
                })();
            lesson_trace::trace_step!(loss_sum);
        }
        // Делим значения, получая нормированную величину или среднее.
        loss_sum / data.len() as f64
    }

    // Шаг: Измеряем loss модели с нулевыми коэффициентами.
    let before: f64 = calculate_binary_cross_entropy_from_logits(&TRAINING_EXAMPLES, 0., 0.);
    lesson_trace::trace_step!(before);
    // Объявляем повторно используемое вычисление `convert_logit_to_probability`; параметры ниже задают его входы.
    fn convert_logit_to_probability(raw_model_score: f64) -> f64 {
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if raw_model_score >= 0. {
            // Делим значения, получая нормированную величину или среднее.
            1. / (1. + approximate_exponential_with_taylor_series(-raw_model_score))
        // Обрабатываем случай, когда предыдущее условие не выполнено.
        } else {
            // Сохраняем рассчитанное значение `prediction_error` для следующих операций.
            let prediction_error: f64 = approximate_exponential_with_taylor_series(raw_model_score);
            lesson_trace::trace_step!(prediction_error);
            // Делим значения, получая нормированную величину или среднее.
            prediction_error / (1. + prediction_error)
        }
    }
    // Устойчивая формула log-loss избегает прямого вычисления log(0).

    // Шаг: Подбираем параметры градиентным спуском.
    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        // Используем подготовленное значение в следующем шаге примера.
        /* Обновляем вес и смещение по градиенту логистической ошибки. */
        // Сохраняем результат этого шага в промежуточный результат.
        let (mut weight, mut bias): (f64, f64) = (0., 0.);
        lesson_trace::trace_step!(weight);
        lesson_trace::trace_step!(bias);
        // 300 проходов достаточно для сходимости весов на этом маленьком наборе.
        // При другой скорости обучения число проходов пришлось бы подобрать заново.
        for epoch in 0..300 {
            // Выполняем встроенный расчёт один раз и сохраняем результат в `(weight_gradient, bias_gradient)`.
            // Производную функции по параметру или вектор таких производных называют gradient.
            let (weight_loss_rate_of_change, bias_loss_rate_of_change): (f64, f64) =
                (|| -> (f64, f64) {
                    // Используем подготовленное значение в следующем шаге примера.
                    /* Для log-loss производная по logit равна sigmoid_activation_of_raw_score(logit) − правильная метка. */
                    // Сохраняем результат этого шага в `data`.
                    let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
                    lesson_trace::trace_step!(data);
                    // Сохраняем рассчитанное значение `weight` для следующих операций.
                    let weight: f64 = weight;
                    lesson_trace::trace_step!(weight);
                    // Сохраняем рассчитанное значение `bias` для следующих операций.
                    let bias: f64 = bias;
                    lesson_trace::trace_step!(bias);
                    // Сохраняем рассчитанное значение `(mut weight_gradient, mut bias_gradient)` для следующих операций.
                    let (mut weight_loss_rate_of_change, mut bias_loss_rate_of_change): (f64, f64) =
                        (0.0, 0.0);
                    lesson_trace::trace_step!(weight_loss_rate_of_change);
                    lesson_trace::trace_step!(bias_loss_rate_of_change);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for &(feature_value, target_label) in data {
                        lesson_trace::trace_step!(feature_value);
                        lesson_trace::trace_step!(target_label);
                        // Сохраняем рассчитанное значение `prediction_error` для следующих операций.
                        let prediction_error: f64 =
                        // Умножаем величины согласно используемой формуле.
                        convert_logit_to_probability(weight * feature_value + bias) - target_label;
                        lesson_trace::trace_step!(prediction_error);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        weight_loss_rate_of_change += prediction_error * feature_value;
                        lesson_trace::trace_step!(weight_loss_rate_of_change);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        bias_loss_rate_of_change += prediction_error;
                        lesson_trace::trace_step!(bias_loss_rate_of_change);
                    }
                    // Составляем результат из вычисленных значений в указанном порядке.
                    (
                        // Делим значения, получая нормированную величину или среднее.
                        weight_loss_rate_of_change / data.len() as f64,
                        // Делим значения, получая нормированную величину или среднее.
                        bias_loss_rate_of_change / data.len() as f64,
                    )
                })();
            lesson_trace::trace_step!(weight_loss_rate_of_change);
            lesson_trace::trace_step!(bias_loss_rate_of_change);
            // Вычитаем очередной вклад из текущего значения параметра.
            // 0.1 — скорость обучения: за шаг меняем вес на десятую часть его градиента.
            weight -= 0.1 * weight_loss_rate_of_change;
            lesson_trace::trace_step!(weight);
            // Вычитаем очередной вклад из текущего значения параметра.
            bias -= 0.1 * bias_loss_rate_of_change;
            lesson_trace::trace_step!(bias);
            if matches!(epoch, 0 | 1 | 9 | 99 | 299) {
                let loss: f64 =
                    calculate_binary_cross_entropy_from_logits(&TRAINING_EXAMPLES, weight, bias);
                println!(
                    "после эпохи {}: вес={weight:.4}, смещение={bias:.4}, log-loss={loss:.4}",
                    epoch + 1
                );
            }
        }
        // Составляем результат из вычисленных значений в указанном порядке.
        (weight, bias)
    })();
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_step!(bias);
    // Шаг: Сравниваем loss до и после обучения и выводим вероятность класса.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "log-loss: {before:.3} -> {:.3}; P(y=1|x=2)={:.3}",
        // Вызываем нужное вычисление с подготовленными аргументами.
        calculate_binary_cross_entropy_from_logits(&TRAINING_EXAMPLES, weight, bias),
        // Умножаем величины согласно используемой формуле.
        convert_logit_to_probability(2. * weight + bias)
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_logistic_regression_with_sigmoid_loss_and_class_threshold(weight, bias);

    // Строим график по результатам урока.
    fn visualize_practice_logistic_regression_with_sigmoid_loss_and_class_threshold(
        weight: f64,
        bias: f64,
    ) {
        // Наглядное представление вычислений сводной практики.
        let model_points: Vec<(f64, f64)> = (-10..=60)
            // Преобразуем каждый элемент в новое значение.
            .map(|plot_step_index| {
                // Сохраняем результат этого шага в `horizontal_value`.
                let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                // Добавляем пару значений для сравнения или построения графика.
                (
                    horizontal_value,
                    1.0 / (1.0 + (-(weight * horizontal_value + bias)).exp()),
                )
            })
            // Собираем результаты в коллекцию.
            .collect();
        // Собираем значения для `training_label_points` в коллекцию.
        let training_label_points: Vec<(f64, f64)> = TRAINING_EXAMPLES
            .iter()
            .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
            .collect();
        // Строим график по рассчитанным значениям и сохраняем его как SVG.
        let chart: std::path::PathBuf = lesson_visualization::line_chart(
            // Передаём путь к каталогу текущего урока.
            env!("CARGO_MANIFEST_DIR"),
            // Указываем имя SVG-файла.
            "lesson-chart",
            // Указываем заголовок графика.
            "Обученная логистическая модель",
            // Указываем подпись горизонтальной оси.
            "признак",
            // Указываем подпись вертикальной оси.
            "P(y=1)",
            // Передаём ряды или значения для отрисовки графика.
            &[
                // Добавляем ряд данных с подписью к графику.
                lesson_visualization::Series {
                    // Указываем подпись этого ряда в легенде.
                    name: "модель",
                    // Передаём рассчитанные координаты точек.
                    points: &model_points,
                },
                // Добавляем ряд данных с подписью к графику.
                lesson_visualization::Series {
                    // Указываем подпись этого ряда в легенде.
                    name: "метки обучения",
                    // Передаём рассчитанные координаты точек.
                    points: &training_label_points,
                },
            ],
        )
        // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
        .expect("не удалось сохранить график");
        // Печатаем путь к созданному SVG, чтобы его можно было открыть.
        println!("график: {}", chart.display());
    }
}
