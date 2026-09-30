// Урок 10.5. Практика: обучение прогнозу вероятности класса и выбор порога.
// Связь с принятой терминологией: Логистическая регрессия, сигмоида, ошибка и порог.
// Зачем здесь эта тема: Классификация требует согласовать логит, вероятность, ошибку обучения и
//   порог решения.
// Почему код устроен так: Проводим одни и те же примеры через все четыре шага и проверяем итоговый
//   класс.
// Представь: Один и тот же пример проходит путь «признаки → логит → вероятность → ошибка и класс».
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
    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::NEG_INFINITY || value < -745.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.0;
        }
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::INFINITY || value > 709.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return f64::INFINITY;
        }
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value < 0.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        lesson_trace::trace_note!("Создаём изменяемое значение `reduced` для следующих операций.");
        let mut reduced: f64 = value;
        lesson_trace::trace_step!(reduced);
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `halving_count` начальным состоянием."
        );
        let mut halving_count: i32 = 0;
        lesson_trace::trace_step!(halving_count);
        lesson_trace::trace_note!(
            "Уменьшаем аргумент до ≤0.5: на таком интервале ряд Тейлора для exp сходится быстро."
        );
        while reduced > 0.5 {
            lesson_trace::trace_note!("Масштабируем текущую величину делением.");
            reduced /= 2.0;
            lesson_trace::trace_step!(reduced);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            halving_count += 1;
            lesson_trace::trace_step!(halving_count);
        }
        lesson_trace::trace_note!("Создаём изменяемое значение `term` для следующих операций.");
        let mut term: f64 = 1.0;
        lesson_trace::trace_step!(term);
        lesson_trace::trace_note!("Создаём изменяемое значение `result` для следующих операций.");
        let mut result: f64 = 1.0;
        lesson_trace::trace_step!(result);
        lesson_trace::trace_note!(
            "Берём 30 членов ряда exp(y)=Σ y^k/k!; это предел приближения для учебных входов."
        );
        for term_index in 1..=30 {
            lesson_trace::trace_step!(term_index);
            lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
            term *= reduced / term_index as f64;
            lesson_trace::trace_step!(term);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            result += term;
            lesson_trace::trace_step!(result);
        }
        lesson_trace::trace_note!(
            "Восстанавливаем exp(value): каждое возведение в квадрат отменяет одно деление аргумента на 2."
        );
        for _ in 0..halving_count {
            lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
            result *= result;
            lesson_trace::trace_step!(result);
        }
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `result` в текущем выражении."
        );
        result
    }

    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    lesson_trace::trace_note!("Учебная пара: вход -3., ожидаемое значение 0..");
    lesson_trace::trace_note!("Учебная пара: вход -2., ожидаемое значение 0..");
    lesson_trace::trace_note!("Учебная пара: вход -1., ожидаемое значение 0..");
    lesson_trace::trace_note!("Учебная пара: вход 1., ожидаемое значение 1..");
    lesson_trace::trace_note!("Учебная пара: вход 2., ожидаемое значение 1..");
    lesson_trace::trace_note!("Учебная пара: вход 3., ожидаемое значение 1..");
    const TRAINING_EXAMPLES: [(f64, f64); 6] = [
        (-3., 0.),
        (-2., 0.),
        (-1., 0.),
        (1., 1.),
        (2., 1.),
        (3., 1.),
    ];

    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `calculate_binary_classification_loss_as_average_negative_log_label_probability`; параметры ниже задают его входы."
    );
    /// Бинарная перекрёстная энтропия: среднее −y·ln(p) − (1−y)·ln(1−p), вычисленное устойчиво из оценок линейной модели.
    fn calculate_binary_classification_loss_as_average_negative_log_label_probability(
        data: &[(f64, f64)],

        weight: f64,

        bias: f64,
    ) -> f64 {
        lesson_trace::trace_note!(
            "Получаем набор наблюдений, по которому считаем ошибку или градиент."
        );
        lesson_trace::trace_note!("Параметр `weight` передаёт коэффициент при признаке.");
        lesson_trace::trace_note!("Параметр `bias` передаёт свободный член модели.");
        lesson_trace::trace_note!("Указываем тип возвращаемого значения.");
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `loss_sum` начальным состоянием."
        );
        let mut loss_sum: f64 = 0.0;
        lesson_trace::trace_step!(loss_sum);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for &(feature_value, target_label) in data {
            lesson_trace::trace_step!(feature_value);
            lesson_trace::trace_step!(target_label);
            lesson_trace::trace_note!(
                "Умножаем значения и сохраняем результат в `raw_model_score`."
            );
            lesson_trace::trace_note!(
                "Оценку модели до преобразования в вероятность называют logit."
            );
            let raw_model_score: f64 = weight * feature_value + bias;
            lesson_trace::trace_step!(raw_model_score);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            lesson_trace::trace_note!(
                "Складываем или вычитаем величины согласно используемой формуле."
            );
            loss_sum += (|| -> f64 {
                lesson_trace::trace_note!(
                    "Используем подготовленное значение в следующем шаге примера."
                );
                lesson_trace::trace_note!(
                    "Выбираем большее из двух чисел для формул softmax, log-loss и Q-learning."
                );
                lesson_trace::trace_note!("Сохраняем результат этого шага в `first`.");
                let first: f64 = raw_model_score;
                lesson_trace::trace_step!(first);
                lesson_trace::trace_note!("Инициализируем значение `second` начальным состоянием.");
                let second: f64 = 0.;
                lesson_trace::trace_step!(second);
                lesson_trace::trace_note!(
                    "Проверяем условие и выбираем соответствующую ветку алгоритма."
                );
                lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
                if first > second { first } else { second }
            })() - target_label * raw_model_score
                + (|| -> f64 {
                    lesson_trace::trace_note!(
                        "Обновляем значение результатом текущего вычисления."
                    );
                    lesson_trace::trace_note!(
                        "ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1)."
                    );
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
                    lesson_trace::trace_note!(
                        "Складываем или вычитаем величины согласно используемой формуле."
                    );
                    let value: f64 = 1.
                        + approximate_e_to_power_by_summing_power_over_factorial_terms(
                            -(|| -> f64 {
                                lesson_trace::trace_note!(
                                    "Используем подготовленное значение в следующем шаге примера."
                                );
                                lesson_trace::trace_note!(
                                    "Модуль числа по определению: меняем знак только у отрицательного числа."
                                );
                                lesson_trace::trace_note!(
                                    "Сохраняем результат этого шага в `value`."
                                );
                                let value: f64 = raw_model_score;
                                lesson_trace::trace_step!(value);
                                lesson_trace::trace_note!(
                                    "Проверяем условие и выбираем соответствующую ветку алгоритма."
                                );
                                if value < 0.0 { -value } else { value }
                            })(),
                        );
                    lesson_trace::trace_step!(value);
                    lesson_trace::trace_note!(
                        "Проверяем обязательное условие до дальнейшего вычисления."
                    );
                    lesson_trace::trace_note!(
                        "Передаём очередное значение в составе результата или вызова."
                    );
                    lesson_trace::trace_note!(
                        "Подставляем результаты в этот шаблон вывода или текстового значения."
                    );
                    assert!(
                        value > 0.0,
                        "логарифм определён только для положительных чисел"
                    );
                    lesson_trace::trace_note!(
                        "Проверяем условие и выбираем соответствующую ветку алгоритма."
                    );
                    if value == f64::INFINITY {
                        lesson_trace::trace_note!(
                            "Завершаем текущий расчёт и возвращаем найденное значение."
                        );
                        return f64::INFINITY;
                    }
                    lesson_trace::trace_note!(
                        "Создаём изменяемое значение `scaled` для следующих операций."
                    );
                    let mut scaled: f64 = value;
                    lesson_trace::trace_step!(scaled);
                    lesson_trace::trace_note!(
                        "Инициализируем изменяемый накопитель `power_of_two` начальным состоянием."
                    );
                    let mut power_of_two: i32 = 0i32;
                    lesson_trace::trace_step!(power_of_two);
                    lesson_trace::trace_note!(
                        "Повторяем вычисление, пока выполняется указанное условие."
                    );
                    while scaled >= 2.0 {
                        lesson_trace::trace_note!("Масштабируем текущую величину делением.");
                        scaled /= 2.0;
                        lesson_trace::trace_step!(scaled);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        power_of_two += 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    lesson_trace::trace_note!(
                        "Повторяем вычисление, пока выполняется указанное условие."
                    );
                    while scaled < 1.0 {
                        lesson_trace::trace_note!(
                            "Умножаем накопленное значение на очередной множитель."
                        );
                        scaled *= 2.0;
                        lesson_trace::trace_step!(scaled);
                        lesson_trace::trace_note!(
                            "Вычитаем очередной вклад из текущего значения параметра."
                        );
                        power_of_two -= 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    lesson_trace::trace_note!(
                        "Этот ряд — учебное раскрытие `value.ln()`; он может работать медленнее и отличаться по точности."
                    );
                    lesson_trace::trace_note!(
                        "Объявляем повторно используемое вычисление `approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers`; параметры ниже задают его входы."
                    );
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        value: f64,
                    ) -> f64 {
                        lesson_trace::trace_note!(
                            "Нормируем или усредняем величину делением и сохраняем её в `ratio`."
                        );
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        lesson_trace::trace_step!(ratio);
                        lesson_trace::trace_note!(
                            "Умножаем значения и сохраняем результат в `ratio_squared`."
                        );
                        let ratio_squared: f64 = ratio * ratio;
                        lesson_trace::trace_step!(ratio_squared);
                        lesson_trace::trace_note!(
                            "Создаём изменяемое значение `term` для следующих операций."
                        );
                        let mut term: f64 = ratio;
                        lesson_trace::trace_step!(term);
                        lesson_trace::trace_note!(
                            "Инициализируем изменяемый накопитель `result` начальным состоянием."
                        );
                        let mut result: f64 = 0.0;
                        lesson_trace::trace_step!(result);
                        lesson_trace::trace_note!(
                            "Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1)."
                        );
                        lesson_trace::trace_note!(
                            "Это конечное приближение: для положительного value выполняется |ratio| < 1."
                        );
                        for term_index in 0..40 {
                            lesson_trace::trace_step!(term_index);
                            lesson_trace::trace_note!(
                                "Прибавляем очередной вклад к ранее накопленному результату."
                            );
                            result += term / (2 * term_index + 1) as f64;
                            lesson_trace::trace_step!(result);
                            lesson_trace::trace_note!(
                                "Умножаем накопленное значение на очередной множитель."
                            );
                            term *= ratio_squared;
                            lesson_trace::trace_step!(term);
                        }
                        lesson_trace::trace_note!(
                            "Умножаем величины согласно используемой формуле."
                        );
                        2.0 * result
                    }
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций."
                    );
                    let logarithm_of_two: f64 =
                        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        );
                    lesson_trace::trace_step!(logarithm_of_two);
                    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64 * logarithm_of_two
                })();
            lesson_trace::trace_step!(loss_sum);
        }
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        loss_sum / data.len() as f64
    }

    lesson_trace::trace_note!("Шаг: Измеряем loss модели с нулевыми коэффициентами.");
    let before: f64 =
        calculate_binary_classification_loss_as_average_negative_log_label_probability(
            &TRAINING_EXAMPLES,
            0.,
            0.,
        );
    lesson_trace::trace_step!(before);
    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score`; параметры ниже задают его входы."
    );
    /// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
    fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
        raw_model_score: f64,
    ) -> f64 {
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if raw_model_score >= 0. {
            lesson_trace::trace_note!(
                "Делим значения, получая нормированную величину или среднее."
            );
            1. / (1.
                + approximate_e_to_power_by_summing_power_over_factorial_terms(-raw_model_score))
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `prediction_error` для следующих операций."
            );
            let prediction_error: f64 =
                approximate_e_to_power_by_summing_power_over_factorial_terms(raw_model_score);
            lesson_trace::trace_step!(prediction_error);
            lesson_trace::trace_note!(
                "Делим значения, получая нормированную величину или среднее."
            );
            prediction_error / (1. + prediction_error)
        }
    }
    lesson_trace::trace_note!("Устойчивая формула log-loss избегает прямого вычисления log(0).");

    lesson_trace::trace_note!("Шаг: Подбираем параметры градиентным спуском.");
    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Обновляем вес и смещение по градиенту логистической ошибки.");
        lesson_trace::trace_note!("Сохраняем результат этого шага в промежуточный результат.");
        let (mut weight, mut bias): (f64, f64) = (0., 0.);
        lesson_trace::trace_step!(weight);
        lesson_trace::trace_step!(bias);
        lesson_trace::trace_note!(
            "300 проходов достаточно для сходимости весов на этом маленьком наборе."
        );
        lesson_trace::trace_note!(
            "При другой скорости обучения число проходов пришлось бы подобрать заново."
        );
        for epoch in 0..300 {
            lesson_trace::trace_note!(
                "Выполняем встроенный расчёт один раз и сохраняем результат в `(weight_gradient, bias_gradient)`."
            );
            lesson_trace::trace_note!(
                "Производную функции по параметру или вектор таких производных называют gradient."
            );
            let (weight_loss_rate_of_change, bias_loss_rate_of_change): (f64, f64) =
                (|| -> (f64, f64) {
                    lesson_trace::trace_note!(
                        "Используем подготовленное значение в следующем шаге примера."
                    );
                    lesson_trace::trace_note!(
                        "Для log-loss производная по logit равна calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(logit) − правильная метка."
                    );
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
                    let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
                    lesson_trace::trace_step!(data);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `weight` для следующих операций."
                    );
                    let weight: f64 = weight;
                    lesson_trace::trace_step!(weight);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `bias` для следующих операций."
                    );
                    let bias: f64 = bias;
                    lesson_trace::trace_step!(bias);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `(mut weight_gradient, mut bias_gradient)` для следующих операций."
                    );
                    let (mut weight_loss_rate_of_change, mut bias_loss_rate_of_change): (f64, f64) =
                        (0.0, 0.0);
                    lesson_trace::trace_step!(weight_loss_rate_of_change);
                    lesson_trace::trace_step!(bias_loss_rate_of_change);
                    lesson_trace::trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for &(feature_value, target_label) in data {
                        lesson_trace::trace_step!(feature_value);
                        lesson_trace::trace_step!(target_label);
                        lesson_trace::trace_note!(
                            "Сохраняем рассчитанное значение `prediction_error` для следующих операций."
                        );
                        lesson_trace::trace_note!(
                            "Умножаем величины согласно используемой формуле."
                        );
                        let prediction_error: f64 =

                        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(weight * feature_value + bias) - target_label;
                        lesson_trace::trace_step!(prediction_error);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        weight_loss_rate_of_change += prediction_error * feature_value;
                        lesson_trace::trace_step!(weight_loss_rate_of_change);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        bias_loss_rate_of_change += prediction_error;
                        lesson_trace::trace_step!(bias_loss_rate_of_change);
                    }
                    lesson_trace::trace_note!(
                        "Составляем результат из вычисленных значений в указанном порядке."
                    );
                    lesson_trace::trace_note!(
                        "Делим значения, получая нормированную величину или среднее."
                    );
                    lesson_trace::trace_note!(
                        "Делим значения, получая нормированную величину или среднее."
                    );
                    (
                        weight_loss_rate_of_change / data.len() as f64,
                        bias_loss_rate_of_change / data.len() as f64,
                    )
                })();
            lesson_trace::trace_step!(weight_loss_rate_of_change);
            lesson_trace::trace_step!(bias_loss_rate_of_change);
            lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
            lesson_trace::trace_note!(
                "0.1 — скорость обучения: за шаг меняем вес на десятую часть его градиента."
            );
            weight -= 0.1 * weight_loss_rate_of_change;
            lesson_trace::trace_step!(weight);
            lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
            bias -= 0.1 * bias_loss_rate_of_change;
            lesson_trace::trace_step!(bias);
            if matches!(epoch, 0 | 1 | 9 | 99 | 299) {
                let loss: f64 =
                    calculate_binary_classification_loss_as_average_negative_log_label_probability(
                        &TRAINING_EXAMPLES,
                        weight,
                        bias,
                    );
                println!(
                    "после эпохи {}: вес={weight:.4}, смещение={bias:.4}, log-loss={loss:.4}",
                    epoch + 1
                );
            }
        }
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        (weight, bias)
    })();
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_step!(bias);
    lesson_trace::trace_note!(
        "Шаг: Сравниваем loss до и после обучения и выводим вероятность класса."
    );
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
    println!(
        "log-loss: {before:.3} -> {:.3}; P(y=1|x=2)={:.3}",
        calculate_binary_classification_loss_as_average_negative_log_label_probability(
            &TRAINING_EXAMPLES,
            weight,
            bias
        ),
        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
            2. * weight + bias
        )
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_predicted_positive_probability_and_training_labels(weight, bias);

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_predicted_positive_probability_and_training_labels(weight: f64, bias: f64) {
        lesson_trace::trace_note!("Наглядное представление вычислений сводной практики.");
        lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
        lesson_trace::trace_note!("Собираем результаты в коллекцию.");
        let model_points: Vec<(f64, f64)> = (-10..=60)
            .map(|plot_step_index| {
                lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
                let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                lesson_trace::trace_note!(
                    "Добавляем пару значений для сравнения или построения графика."
                );
                (
                    horizontal_value,
                    1.0 / (1.0 + (-(weight * horizontal_value + bias)).exp()),
                )
            })
            .collect();
        lesson_trace::trace_note!("Собираем значения для `training_label_points` в коллекцию.");
        let training_label_points: Vec<(f64, f64)> = TRAINING_EXAMPLES
            .iter()
            .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
            .collect();
        lesson_trace::trace_note!(
            "Строим график по рассчитанным значениям и сохраняем его как SVG."
        );
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок графика.");
        lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
        lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
        lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
        lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
        lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
        lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
        lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
        lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
        lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Обученная логистическая модель",
            "признак",
            "P(y=1)",
            &[
                lesson_visualization::Series {
                    name: "модель",

                    points: &model_points,
                },
                lesson_visualization::Series {
                    name: "метки обучения",

                    points: &training_label_points,
                },
            ],
        )
        .expect("не удалось сохранить график");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
