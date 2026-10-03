//! Урок 051. Измеряем среднюю величину промаха прогноза.
//! Из прогноза вычитаем правильный ответ и берём величину разницы без знака.
//! Складываем результаты и делим на число примеров. Ноль означает точные прогнозы.
//! Если прогнозы ошиблись на −2 и +4, средняя величина ошибки равна (2+4)/2 = 3.

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

/// Средний модуль ошибки с проверкой числа пар.
/// Средняя абсолютная ошибка (MAE): суммируем модули разностей прогноза и ответа, делим на число пар.
pub fn calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(
    targets: &[f64],
    predictions: &[f64],
) -> Result<f64, &'static str> {
    validate_equal_lens_of_targets_and_predictions(targets, predictions)?;
    let mut absolute_sum: f64 = 0.0;
    for index in 0..targets.len() {
        let error: f64 = predictions[index] - targets[index];
        absolute_sum += if error < 0.0 { -error } else { error };
    }
    Ok(absolute_sum / targets.len() as f64)
}
