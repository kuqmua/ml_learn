//! Урок 050. Средняя квадратичная ошибка прогноза: сумма квадратов ошибок, делённая на число примеров.

fn validate_equal_lens_of_targets_and_predictions(
    targets: &[f64],
    predictions: &[f64],
) -> Result<(), &'static str> {
    if targets.len() != predictions.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    if targets.is_empty() {
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    Ok(())
}

/// Средний квадрат ошибки с проверкой числа пар.
/// Средняя квадратичная ошибка (MSE): суммируем квадраты разностей прогноза и ответа, делим на число пар.
pub fn calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse(
    targets: &[f64],
    predictions: &[f64],
) -> Result<f64, &'static str> {
    validate_equal_lens_of_targets_and_predictions(targets, predictions)?;
    let mut squared_sum: f64 = 0.0;
    for index in 0..targets.len() {
        let error: f64 = predictions[index] - targets[index];
        squared_sum += error * error;
    }
    Ok(squared_sum / targets.len() as f64)
}
