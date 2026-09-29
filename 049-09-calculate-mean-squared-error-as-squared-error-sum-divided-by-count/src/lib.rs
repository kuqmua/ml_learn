//! Урок 049. Средняя квадратичная ошибка прогноза: сумма квадратов ошибок, делённая на число примеров.

fn validate_equal_lengths_of_targets_and_predictions(
    targets: &[f64],
    predictions: &[f64],
) -> Result<(), &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if targets.len() != predictions.len() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    // Выбираем дальнейший шаг по выполнению условия.
    if targets.is_empty() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    // Возвращаем успешный результат.
    Ok(())
}

/// Средний квадрат ошибки с проверкой числа пар.
/// Средняя квадратичная ошибка (MSE): суммируем квадраты разностей прогноза и ответа, делим на число пар.
pub fn calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
    targets: &[f64],
    predictions: &[f64],
) -> Result<f64, &'static str> {
    // Используем подготовленное значение в следующем шаге примера.
    validate_equal_lengths_of_targets_and_predictions(targets, predictions)?;
    // Сохраняем результат этого шага в `squared_sum`.
    let mut squared_sum: f64 = 0.0;
    lesson_trace::trace_step!(squared_sum);
    // Повторяем расчёт для каждого элемента последовательности.
    for index in 0..targets.len() {
        lesson_trace::trace_step!(index);
        // Сохраняем результат этого шага в `error`.
        let error: f64 = predictions[index] - targets[index];
        lesson_trace::trace_step!(error);
        // Обновляем значение результатом текущего вычисления.
        squared_sum += error * error;
        lesson_trace::trace_step!(squared_sum);
    }
    // Возвращаем успешный результат.
    Ok(squared_sum / targets.len() as f64)
}
