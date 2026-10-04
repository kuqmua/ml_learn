// Урок 240. Соединять оценку изменения распределения, качество по новым ответам, время прогноза и
// порог предупреждения.
// Сравниваем неизменный и сдвинутый наборы и показываем размеры выборок вместе с полученными
// показателями.

fn main() {
    /// Выбираем большее из двух чисел для формул softmax, log-loss и Q-learning.
    /// Аналог `number1.max(number2)` для обычных чисел; при NaN результат может отличаться.
    fn choose_larger_number(number1: f64, number2: f64) -> f64 {
        if number1 > number2 { number1 } else { number2 }
    }

    fn calc_shares_of_feature_values_in_three_bins(data: &[f64]) -> [f64; 3] {
        let mut bin_counts: [f64; 3] = [0.0; 3];
        for &feature_value in data {
            let histogram_bin: usize = if feature_value < 0.0 {
                0
            } else if feature_value < 1.0 {
                1
            } else {
                2
            };
            bin_counts[histogram_bin] += 1.0;
        }
        for group_share in &mut bin_counts {
            *group_share /= data.len() as f64;
        }
        bin_counts
    }
    /// Индекс стабильности популяции (PSI): суммируем (current−reference)·ln(current/reference) по долям трёх интервалов, ограничивая доли снизу.
    fn calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios(
        reference: &[f64],
        current: &[f64],
    ) -> f64 {
        let reference_group_shares: [f64; 3] =
            calc_shares_of_feature_values_in_three_bins(reference);
        let current_group_shares: [f64; 3] = calc_shares_of_feature_values_in_three_bins(current);
        let mut distribution_shift_score: f64 = 0.0;
        for bin_index in 0..reference_group_shares.len() {
            let reference_group_share: f64 =
                choose_larger_number(reference_group_shares[bin_index], 1e-6);
            let current_group_share: f64 =
                choose_larger_number(current_group_shares[bin_index], 1e-6);
            distribution_shift_score += (current_group_share - reference_group_share)
                * (|| -> f64 {
                    let value: f64 = current_group_share / reference_group_share;
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
        distribution_shift_score
    }

    let reference: [f64; 6] = [-1.0, -0.5, 0.1, 0.2, 1.2, 1.5];
    let stable: [f64; 6] = reference;
    let shifted: [f64; 6] = [1.1, 1.2, 1.3, 1.4, 1.5, 1.6];
    let _ = (
        &(reference.len()),
        &(shifted.len()),
        &(calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios(
            &reference, &stable,
        )),
        &(calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios(
            &reference, &shifted,
        )),
    );

    // Выполняем вычисления из примера.
    let _ = (&reference, &stable, &shifted);

    let stable_score =
        calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios(
            &reference, &stable,
        );
    let shifted_score =
        calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios(
            &reference, &shifted,
        );
    assert!(stable_score.abs() < 1e-12);
    assert!(shifted_score > 0.2);
    let targets = [true, false, true, false];
    let features = [-1.0_f64, -2.0, 3.0, 4.0];
    let start = std::time::Instant::now();
    let predictions = std::hint::black_box(features).map(|x| x >= 0.0);
    std::hint::black_box(&predictions);
    let duration = start.elapsed();
    let correct = targets
        .iter()
        .zip(predictions)
        .filter(|(a, b)| **a == *b)
        .count();
    println!(
        "Наблюдений эталона={}, новых={}; сдвиг без изменений={stable_score}, после изменения={shifted_score}",
        reference.len(),
        shifted.len()
    );
    println!(
        "Свежих размеченных примеров={}, точность={}; время прогнозов={duration:?}; предупреждение о сдвиге={}",
        targets.len(),
        correct as f64 / targets.len() as f64,
        shifted_score > 0.2
    );
    assert_eq!(correct, 2);
}

// Чему учит этот урок:
// Учимся соединять оценку изменения распределения, качество по новым ответам, время прогноза и
// порог предупреждения.
// Сравниваем неизменный и сдвинутый наборы и показываем размеры выборок вместе с полученными
// показателями.
