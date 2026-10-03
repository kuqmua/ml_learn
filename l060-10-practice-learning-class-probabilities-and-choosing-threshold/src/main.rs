// Урок 10.5. Практика: обучение прогнозу вероятности класса и выбор порога.
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
        let mut exponential_approximation: f64 = 1.0;
        for term_index in 1..=30 {
            term *= reduced / term_index as f64;
            exponential_approximation += term;
        }
        for _ in 0..halving_count {
            exponential_approximation *= exponential_approximation;
        }
        exponential_approximation
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
    fn calc_binary_classification_loss_as_average_neg_log_target_probability_where_closer_to_0_means_more_probability_on_correct_answers(
        data: &[(f64, f64)],

        weight: f64,

        constant_input_weight: f64,
    ) -> f64 {
        let mut loss_sum: f64 = 0.0;
        for &(feature_value, target) in data {
            let raw_model_score: f64 = weight * feature_value + constant_input_weight;
            loss_sum += (|| -> f64 {
                let raw_score_to_compare: f64 = raw_model_score;
                let zero_to_compare: f64 = 0.;
                if raw_score_to_compare > zero_to_compare {
                    raw_score_to_compare
                } else {
                    zero_to_compare
                }
            })() - target * raw_model_score
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
                        let mut logarithm_series_sum: f64 = 0.0;
                        for term_index in 0..40 {
                            logarithm_series_sum += term / (2 * term_index + 1) as f64;
                            term *= ratio_squared;
                        }
                        2.0 * logarithm_series_sum
                    }

                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64
                        * approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        )
                })();
        }
        loss_sum / data.len() as f64
    }

    let _: f64 = calc_binary_classification_loss_as_average_neg_log_target_probability_where_closer_to_0_means_more_probability_on_correct_answers(
        &TRAINING_EXAMPLES,
        0.,
        0.,
    );
    /// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
    fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(
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

    let (weight, constant_input_weight): (f64, f64) = (|| -> (f64, f64) {
        let (mut weight, mut constant_input_weight): (f64, f64) = (0., 0.);
        for epoch in 0..300 {
            let (weight_loss_rate_of_change, constant_input_weight_loss_rate_of_change): (
                f64,
                f64,
            ) = (|| -> (f64, f64) {
                let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
                let weight: f64 = weight;
                let constant_input_weight: f64 = constant_input_weight;
                let (mut weight_loss_rate_of_change, mut constant_input_weight_loss_rate_of_change): (f64, f64) =
                        (0.0, 0.0);
                for &(feature_value, target) in data {
                    let prediction_error: f64 =

                        calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(weight * feature_value + constant_input_weight) - target;
                    weight_loss_rate_of_change += prediction_error * feature_value;
                    constant_input_weight_loss_rate_of_change += prediction_error;
                }
                (
                    weight_loss_rate_of_change / data.len() as f64,
                    constant_input_weight_loss_rate_of_change / data.len() as f64,
                )
            })();
            weight -= 0.1 * weight_loss_rate_of_change;
            constant_input_weight -= 0.1 * constant_input_weight_loss_rate_of_change;
            if matches!(epoch, 0 | 1 | 9 | 99 | 299) {
                let _: f64 =
                    calc_binary_classification_loss_as_average_neg_log_target_probability_where_closer_to_0_means_more_probability_on_correct_answers(
                        &TRAINING_EXAMPLES,
                        weight,
                        constant_input_weight,
                    );
                let _ = &(epoch + 1);
            }
        }
        (weight, constant_input_weight)
    })();
    let _ = (
        &(calc_binary_classification_loss_as_average_neg_log_target_probability_where_closer_to_0_means_more_probability_on_correct_answers(
            &TRAINING_EXAMPLES,
            weight,
            constant_input_weight,
        )),
        &(calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(
            2. * weight + constant_input_weight,
        )),
    );

    plot_predicted_pos_probability_and_training_targets(weight, constant_input_weight);

    fn plot_predicted_pos_probability_and_training_targets(
        weight: f64,
        constant_input_weight: f64,
    ) {
        lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Обученная логистическая модель",
            "признак",
            "P(y=1)",
            &[
                lesson_visualization::Series {
                    name: "модель",

                    points: &(-10..=60)
                        .map(|plot_step_index| {
                            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                            (
                                horizontal_value,
                                1.0 / (1.0
                                    + (-(weight * horizontal_value + constant_input_weight)).exp()),
                            )
                        })
                        .collect::<Vec<_>>(),
                },
                lesson_visualization::Series {
                    name: "метки обучения",

                    points: &TRAINING_EXAMPLES
                        .iter()
                        .map(|&(horizontal_value, vertical_value)| {
                            (horizontal_value, vertical_value)
                        })
                        .collect::<Vec<_>>(),
                },
            ],
        )
        .expect("не удалось сохранить график");
    }
}
