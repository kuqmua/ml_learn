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
//   спуском. Сравниваем loss до и после обучения и вычисляем вероятность класса.
// Что проверить при изменении примера: Проверь вероятности в [0,1], улучшение loss и влияние порога на
//   precision/recall.
// Дополнительная практика: Реализуй бинарный классификатор и стабильный расчёт log-loss без log(0).

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        if value == f64::NEG_INFINITY || value < -745.0 {
            return 0.0;
        }
        if value == f64::INFINITY || value > 709.0 {
            return f64::INFINITY;
        }
        if value < 0.0 {
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        let mut reduced: f64 = value;
        let mut halving_count: i32 = 0;
        while reduced > 0.5 {
            reduced /= 2.0;
            halving_count += 1;
        }
        let mut term: f64 = 1.0;
        let mut result: f64 = 1.0;
        for term_index in 1..=30 {
            term *= reduced / term_index as f64;
            result += term;
        }
        for _ in 0..halving_count {
            result *= result;
        }
        result
    }

    const TRAINING_EXAMPLES: [(f64, f64); 6] = [
        (-3., 0.),
        (-2., 0.),
        (-1., 0.),
        (1., 1.),
        (2., 1.),
        (3., 1.),
    ];

    /// Бинарная перекрёстная энтропия: среднее −y·ln(p) − (1−y)·ln(1−p), вычисленное устойчиво из оценок линейной модели.
    fn calculate_binary_classification_loss_as_average_negative_log_label_probability(
        data: &[(f64, f64)],

        weight: f64,

        bias: f64,
    ) -> f64 {
        let mut loss_sum: f64 = 0.0;
        for &(feature_value, target_label) in data {
            let raw_model_score: f64 = weight * feature_value + bias;
            loss_sum += (|| -> f64 {
                let first: f64 = raw_model_score;
                let second: f64 = 0.;
                if first > second { first } else { second }
            })() - target_label * raw_model_score
                + (|| -> f64 {
                    let value: f64 = 1.
                        + approximate_e_to_power_by_summing_power_over_factorial_terms(
                            -(|| -> f64 {
                                let value: f64 = raw_model_score;
                                if value < 0.0 { -value } else { value }
                            })(),
                        );
                    assert!(
                        value > 0.0,
                        "логарифм определён только для положительных чисел"
                    );
                    if value == f64::INFINITY {
                        return f64::INFINITY;
                    }
                    let mut scaled: f64 = value;
                    let mut power_of_two: i32 = 0i32;
                    while scaled >= 2.0 {
                        scaled /= 2.0;
                        power_of_two += 1;
                    }
                    while scaled < 1.0 {
                        scaled *= 2.0;
                        power_of_two -= 1;
                    }
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        value: f64,
                    ) -> f64 {
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        let ratio_squared: f64 = ratio * ratio;
                        let mut term: f64 = ratio;
                        let mut result: f64 = 0.0;
                        for term_index in 0..40 {
                            result += term / (2 * term_index + 1) as f64;
                            term *= ratio_squared;
                        }
                        2.0 * result
                    }
                    let logarithm_of_two: f64 =
                        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        );
                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64 * logarithm_of_two
                })();
        }
        loss_sum / data.len() as f64
    }

    let _before: f64 =
        calculate_binary_classification_loss_as_average_negative_log_label_probability(
            &TRAINING_EXAMPLES,
            0.,
            0.,
        );
    /// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
    fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
        raw_model_score: f64,
    ) -> f64 {
        if raw_model_score >= 0. {
            1. / (1.
                + approximate_e_to_power_by_summing_power_over_factorial_terms(-raw_model_score))
        } else {
            let prediction_error: f64 =
                approximate_e_to_power_by_summing_power_over_factorial_terms(raw_model_score);
            prediction_error / (1. + prediction_error)
        }
    }

    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        let (mut weight, mut bias): (f64, f64) = (0., 0.);
        for epoch in 0..300 {
            let (weight_loss_rate_of_change, bias_loss_rate_of_change): (f64, f64) =
                (|| -> (f64, f64) {
                    let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
                    let weight: f64 = weight;
                    let bias: f64 = bias;
                    let (mut weight_loss_rate_of_change, mut bias_loss_rate_of_change): (f64, f64) =
                        (0.0, 0.0);
                    for &(feature_value, target_label) in data {
                        let prediction_error: f64 =

                        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(weight * feature_value + bias) - target_label;
                        weight_loss_rate_of_change += prediction_error * feature_value;
                        bias_loss_rate_of_change += prediction_error;
                    }
                    (
                        weight_loss_rate_of_change / data.len() as f64,
                        bias_loss_rate_of_change / data.len() as f64,
                    )
                })();
            weight -= 0.1 * weight_loss_rate_of_change;
            bias -= 0.1 * bias_loss_rate_of_change;
            if matches!(epoch, 0 | 1 | 9 | 99 | 299) {
                let _loss: f64 =
                    calculate_binary_classification_loss_as_average_negative_log_label_probability(
                        &TRAINING_EXAMPLES,
                        weight,
                        bias,
                    );
                let _ = &(epoch + 1);
            }
        }
        (weight, bias)
    })();
    let _ = (
        &(calculate_binary_classification_loss_as_average_negative_log_label_probability(
            &TRAINING_EXAMPLES,
            weight,
            bias,
        )),
        &(calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
            2. * weight + bias,
        )),
    );

    plot_predicted_positive_probability_and_training_labels(weight, bias);

    fn plot_predicted_positive_probability_and_training_labels(weight: f64, bias: f64) {
        let model_points: Vec<(f64, f64)> = (-10..=60)
            .map(|plot_step_index| {
                let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                (
                    horizontal_value,
                    1.0 / (1.0 + (-(weight * horizontal_value + bias)).exp()),
                )
            })
            .collect();
        let training_label_points: Vec<(f64, f64)> = TRAINING_EXAMPLES
            .iter()
            .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
            .collect();
        let _chart: std::path::PathBuf = lesson_visualization::line_chart(
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
    }
}
