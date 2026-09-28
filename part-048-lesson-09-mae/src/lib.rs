//! Вычисления и примеры урока part-048-lesson-09-mae.

fn validate_prediction_pairs(targets: &[f64], predictions: &[f64]) -> Result<(), &'static str> {
    if targets.len() != predictions.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    if targets.is_empty() {
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    Ok(())
}

/// Средний модуль ошибки с проверкой числа пар.
pub fn mean_absolute_error(targets: &[f64], predictions: &[f64]) -> Result<f64, &'static str> {
    validate_prediction_pairs(targets, predictions)?;
    let mut absolute_sum = 0.0;
    for index in 0..targets.len() {
        let error = predictions[index] - targets[index];
        absolute_sum += if error < 0.0 { -error } else { error };
    }
    Ok(absolute_sum / targets.len() as f64)
}
