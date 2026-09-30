//! Урок 051. Средняя абсолютная ошибка прогноза: сумма модулей ошибок, делённая на число примеров.

fn validate_equal_lengths_of_targets_and_predictions(
    targets: &[f64],
    predictions: &[f64],
) -> Result<(), &'static str> {
    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if targets.len() != predictions.len() {
        lesson_trace::trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if targets.is_empty() {
        lesson_trace::trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    lesson_trace::trace_note!("Возвращаем успешный результат.");
    Ok(())
}

/// Средний модуль ошибки с проверкой числа пар.
/// Средняя абсолютная ошибка (MAE): суммируем модули разностей прогноза и ответа, делим на число пар.
pub fn calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(
    targets: &[f64],
    predictions: &[f64],
) -> Result<f64, &'static str> {
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    validate_equal_lengths_of_targets_and_predictions(targets, predictions)?;
    lesson_trace::trace_note!("Сохраняем результат этого шага в `absolute_sum`.");
    let mut absolute_sum: f64 = 0.0;
    lesson_trace::trace_step!(absolute_sum);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for index in 0..targets.len() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `error`.");
        let error: f64 = predictions[index] - targets[index];
        lesson_trace::trace_step!(error);
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        absolute_sum += if error < 0.0 { -error } else { error };
        lesson_trace::trace_step!(absolute_sum);
    }
    lesson_trace::trace_note!("Возвращаем успешный результат.");
    Ok(absolute_sum / targets.len() as f64)
}
