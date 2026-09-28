//! Вычисления и примеры урока part-047-lesson-09-mse.

fn validate_prediction_pairs(targets: &[f64], predictions: &[f64]) -> Result<(), &'static str> {
    if targets.len() != predictions.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    if targets.is_empty() {
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    Ok(())
}

/// Средний квадрат ошибки с проверкой числа пар.
pub fn mean_squared_error(targets: &[f64], predictions: &[f64]) -> Result<f64, &'static str> {
    validate_prediction_pairs(targets, predictions)?;
    let mut squared_sum = 0.0;
    for index in 0..targets.len() {
        let error = predictions[index] - targets[index];
        squared_sum += error * error;
    }
    Ok(squared_sum / targets.len() as f64)
}
