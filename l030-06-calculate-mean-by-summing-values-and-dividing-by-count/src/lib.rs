//! Урок 030. Среднее арифметическое: сложение значений и деление суммы на их количество.

/// Среднее непустого набора.
/// Среднее арифметическое: складываем значения и делим на их количество.
pub fn calculate_mean_by_summing_values_and_dividing_by_count(
    values: &[f64],
) -> Result<f64, &'static str> {
    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if values.is_empty() {
        lesson_trace::trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("для среднего нужно хотя бы одно значение");
    }
    lesson_trace::trace_note!("Сохраняем результат этого шага в `sum`.");
    let mut sum: f64 = 0.0;
    lesson_trace::trace_step!(sum);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for &value in values {
        lesson_trace::trace_step!(value);
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        sum += value;
        lesson_trace::trace_step!(sum);
    }
    lesson_trace::trace_note!("Возвращаем успешный результат.");
    Ok(sum / values.len() as f64)
}
