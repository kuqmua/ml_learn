//! Урок 032. Измеряем разброс значений относительно их среднего.
//! Из каждого значения вычитаем среднее, каждую разницу умножаем саму на себя и складываем.
//! Делим сумму на количество значений минус один: так оцениваем разброс по выборке.
//! Если все значения одинаковые, результат равен нулю. Чем больше результат, тем сильнее разброс.
//! Для расчёта нужны как минимум два значения.

use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;

pub fn calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one_where_0_means_all_values_equal_and_larger_means_more_spread(
    values: &[f64],
) -> Result<f64, &'static str> {
    if values.len() < 2 {
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    let average: f64 = calc_mean_by_summing_values_and_dividing_by_count(values)?;
    let mut sum_of_squared_diffs_from_mean: f64 = 0.0;
    for &value in values {
        let diff_from_mean: f64 = value - average;
        sum_of_squared_diffs_from_mean += diff_from_mean * diff_from_mean;
    }
    Ok(sum_of_squared_diffs_from_mean / (values.len() - 1) as f64)
}
