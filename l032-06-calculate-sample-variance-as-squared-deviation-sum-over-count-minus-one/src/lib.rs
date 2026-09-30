//! Урок 032. Разброс значений (выборочная дисперсия): сумма квадратов отклонений от среднего, делённая на число значений минус один.

/// Выборочная дисперсия использует среднее из урока 06.1.
/// Выборочная дисперсия: сумму квадратов отклонений от среднего делим на (число значений − 1).
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

pub fn calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(
    values: &[f64],
) -> Result<f64, &'static str> {
    if values.len() < 2 {
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    let average: f64 = calculate_mean_by_summing_values_and_dividing_by_count(values)?;
    let mut squared_deviation_sum: f64 = 0.0;
    for &value in values {
        let deviation: f64 = value - average;
        squared_deviation_sum += deviation * deviation;
    }
    Ok(squared_deviation_sum / (values.len() - 1) as f64)
}
