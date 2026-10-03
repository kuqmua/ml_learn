// Урок 45.5. Практика: наблюдение за изменениями данных, ошибками, временем ответа и предупреждениями.
// Зачем здесь эта тема: Наблюдение модели после выпуска требует вместе видеть входы, метки,
//   задержку и пороги.
// Почему код устроен так: Собираем показатели в одном примере, чтобы предупреждение можно было
//   объяснить исходными числами.
// Представь: Один отчёт показывает дрейф признаков, новую ошибку и задержку, чтобы решение об
//   обновлении имело основания.
//
// Что повторяем вместе: распределения признаков, качество после релиза, латентность, алерты.
// Зачем это нужно: Мониторинг сравнивает новые данные с эталоном, чтобы заметить изменение распределения
//   входного признака.
// Что показывает программа: Задаём эталонное распределение признака. Готовим контрольный набор без сдвига и
//   набор с сильным сдвигом. Сравниваем PSI при одинаковом и изменившемся распределении.
// Что проверить при изменении примера: Проверь сценарий без дрейфа и искусственный сдвиг; отчёт указывает
//   размер выборки.
// Дополнительная практика: Сравни эталонные и новые данные, рассчитай простую метрику дрейфа и статистику
//   ошибок.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    /// Выбираем большее из двух чисел для формул softmax, log-loss и Q-learning.
    /// Аналог `first_number.max(second_number)` для обычных чисел; при NaN результат может отличаться.
    fn choose_larger_number(first_number: f64, second_number: f64) -> f64 {
        if first_number > second_number {
            first_number
        } else {
            second_number
        }
    }

    fn calc_shares_of_feature_values_in_three_bins(data: &[f64]) -> [f64; 3] {
        let mut bin_counts: [f64; 3] = [0.; 3];
        for &feature_value in data {
            let histogram_bin: usize = if feature_value < 0. {
                0
            } else if feature_value < 1. {
                1
            } else {
                2
            };
            bin_counts[histogram_bin] += 1.;
        }
        for group_share in &mut bin_counts {
            *group_share /= data.len() as f64;
        }
        bin_counts
    }
    /// Индекс стабильности популяции (PSI): суммируем (current−reference)·ln(current/reference) по долям трёх интервалов, ограничивая доли снизу.
    fn calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios_where_0_means_matching_bin_shares_and_larger_means_more_change(
        reference: &[f64],
        current: &[f64],
    ) -> f64 {
        let reference_group_shares: [f64; 3] =
            calc_shares_of_feature_values_in_three_bins(reference);
        let current_group_shares: [f64; 3] = calc_shares_of_feature_values_in_three_bins(current);
        let mut distribution_shift_score_where_0_means_matching_bin_shares_and_larger_means_more_change: f64 = 0.0;
        for bin_index in 0..reference_group_shares.len() {
            let reference_group_share: f64 =
                choose_larger_number(reference_group_shares[bin_index], 1e-6);
            let current_group_share: f64 =
                choose_larger_number(current_group_shares[bin_index], 1e-6);
            distribution_shift_score_where_0_means_matching_bin_shares_and_larger_means_more_change += (current_group_share - reference_group_share)
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
        distribution_shift_score_where_0_means_matching_bin_shares_and_larger_means_more_change
    }

    let reference: [f64; 6] = [-1., -0.5, 0.1, 0.2, 1.2, 1.5];
    let stable: [f64; 6] = reference;
    let shifted: [f64; 6] = [1.1, 1.2, 1.3, 1.4, 1.5, 1.6];
    let _ = (&(reference.len()), &(shifted.len()), &(calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios_where_0_means_matching_bin_shares_and_larger_means_more_change(
            &reference, &stable
        )), &(calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios_where_0_means_matching_bin_shares_and_larger_means_more_change(
            &reference, &shifted
        )));

    plot_distribution_shift_score_as_sum_of_interval_share_changes_times_log_share_ratios(
        reference, stable, shifted,
    );

    fn plot_distribution_shift_score_as_sum_of_interval_share_changes_times_log_share_ratios(
        reference: [f64; 6],
        stable: [f64; 6],
        shifted: [f64; 6],
    ) {
        lesson_visualization::bar_chart(

            env!("CARGO_MANIFEST_DIR"),

            "lesson-chart",

            "Сдвиг признака",

            "PSI",

            &[
                (

                    "стабильно",

                    calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios_where_0_means_matching_bin_shares_and_larger_means_more_change(&reference, &stable),
                ),
                (

                    "сдвиг",

                    calc_distribution_shift_score_as_sum_of_bin_share_diffs_times_log_share_ratios_where_0_means_matching_bin_shares_and_larger_means_more_change(&reference, &shifted),
                ),
            ],
        )

        .expect("не удалось сохранить график");
    }
}
