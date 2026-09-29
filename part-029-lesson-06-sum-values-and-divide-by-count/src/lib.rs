//! Урок 029. Сложение значений и деление суммы на их количество.

/// Среднее непустого набора.
/// Среднее арифметическое: складываем значения и делим на их количество.
pub fn calculate_mean_by_summing_values_and_dividing_by_count(
    values: &[f64],
) -> Result<f64, &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if values.is_empty() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("для среднего нужно хотя бы одно значение");
    }
    // Сохраняем результат этого шага в `sum`.
    let mut sum: f64 = 0.0;
    lesson_trace::trace_step!(sum);
    // Повторяем расчёт для каждого элемента последовательности.
    for &value in values {
        lesson_trace::trace_step!(value);
        // Обновляем значение результатом текущего вычисления.
        sum += value;
        lesson_trace::trace_step!(sum);
    }
    // Возвращаем успешный результат.
    Ok(sum / values.len() as f64)
}
