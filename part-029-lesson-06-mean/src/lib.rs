//! Вычисления и примеры урока part-029-lesson-06-mean.

/// Среднее непустого набора.
pub fn mean(values: &[f64]) -> Result<f64, &'static str> {
    if values.is_empty() {
        return Err("для среднего нужно хотя бы одно значение");
    }
    let mut sum = 0.0;
    for &value in values {
        sum += value;
    }
    Ok(sum / values.len() as f64)
}
