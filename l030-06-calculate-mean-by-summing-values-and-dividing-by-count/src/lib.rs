//! Урок 030. Среднее арифметическое: сложение значений и деление суммы на их количество.

/// Среднее непустого набора.
/// Среднее арифметическое: складываем значения и делим на их количество.
use lesson_trace::{trace_note, trace_step};

pub fn calculate_mean_by_summing_values_and_dividing_by_count(
    values: &[f64],
) -> Result<f64, &'static str> {
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if values.is_empty() {
        trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("для среднего нужно хотя бы одно значение");
    }
    trace_note!("Сохраняем результат этого шага в `sum`.");
    let mut sum: f64 = 0.0;
    trace_step!(sum);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for &value in values {
        trace_step!(value);
        trace_note!("Обновляем значение результатом текущего вычисления.");
        sum += value;
        trace_step!(sum);
    }
    trace_note!("Возвращаем успешный результат.");
    Ok(sum / values.len() as f64)
}
