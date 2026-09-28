//! Вычисления и примеры урока part-031-lesson-06-variance.

/// Выборочная дисперсия использует среднее из урока 06.1.
pub fn sample_variance(values: &[f64]) -> Result<f64, &'static str> {
    if values.len() < 2 {
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    let average = lesson_029::mean(values)?;
    let mut squared_deviation_sum = 0.0;
    for &value in values {
        let deviation = value - average;
        squared_deviation_sum += deviation * deviation;
    }
    Ok(squared_deviation_sum / (values.len() - 1) as f64)
}
