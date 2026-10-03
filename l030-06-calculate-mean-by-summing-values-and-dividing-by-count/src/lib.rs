//! Урок 030. Среднее арифметическое: сложение значений и деление суммы на их количество.

/// Среднее непустого набора.
/// Среднее арифметическое: складываем значения и делим на их количество.

pub fn calc_mean_by_summing_values_and_dividing_by_count(
    values: &[f64],
) -> Result<f64, &'static str> {
    if values.is_empty() {
        return Err("для среднего нужно хотя бы одно значение");
    }
    let mut sum_of_values: f64 = 0.0;
    for &value in values {
        sum_of_values += value;
    }
    Ok(sum_of_values / values.len() as f64)
}
