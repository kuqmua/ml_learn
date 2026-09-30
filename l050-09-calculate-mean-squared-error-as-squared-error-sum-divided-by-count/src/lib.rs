//! Урок 050. Средняя квадратичная ошибка прогноза: сумма квадратов ошибок, делённая на число примеров.

use lesson_trace::{trace_note, trace_step};

fn validate_equal_lengths_of_targets_and_predictions(
    targets: &[f64],
    predictions: &[f64],
) -> Result<(), &'static str> {
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if targets.len() != predictions.len() {
        trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if targets.is_empty() {
        trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    trace_note!("Возвращаем успешный результат.");
    Ok(())
}

/// Средний квадрат ошибки с проверкой числа пар.
/// Средняя квадратичная ошибка (MSE): суммируем квадраты разностей прогноза и ответа, делим на число пар.
pub fn calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
    targets: &[f64],
    predictions: &[f64],
) -> Result<f64, &'static str> {
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    validate_equal_lengths_of_targets_and_predictions(targets, predictions)?;
    trace_note!("Сохраняем результат этого шага в `squared_sum`.");
    let mut squared_sum: f64 = 0.0;
    trace_step!(squared_sum);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for index in 0..targets.len() {
        trace_step!(index);
        trace_note!("Сохраняем результат этого шага в `error`.");
        let error: f64 = predictions[index] - targets[index];
        trace_step!(error);
        trace_note!("Обновляем значение результатом текущего вычисления.");
        squared_sum += error * error;
        trace_step!(squared_sum);
    }
    trace_note!("Возвращаем успешный результат.");
    Ok(squared_sum / targets.len() as f64)
}
