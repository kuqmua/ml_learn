//! Урок 032. Разброс значений (выборочная дисперсия): сумма квадратов отклонений от среднего, делённая на число значений минус один.

/// Выборочная дисперсия использует среднее из урока 06.1.
/// Выборочная дисперсия: сумму квадратов отклонений от среднего делим на (число значений − 1).
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

use lesson_trace::{trace_note, trace_step};

pub fn calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(
    values: &[f64],
) -> Result<f64, &'static str> {
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if values.len() < 2 {
        trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    trace_note!("Сохраняем результат этого шага в `average`.");
    let average: f64 = calculate_mean_by_summing_values_and_dividing_by_count(values)?;
    trace_step!(average);
    trace_note!("Сохраняем результат этого шага в `squared_deviation_sum`.");
    let mut squared_deviation_sum: f64 = 0.0;
    trace_step!(squared_deviation_sum);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for &value in values {
        trace_step!(value);
        trace_note!("Сохраняем результат этого шага в `deviation`.");
        let deviation: f64 = value - average;
        trace_step!(deviation);
        trace_note!("Обновляем значение результатом текущего вычисления.");
        squared_deviation_sum += deviation * deviation;
        trace_step!(squared_deviation_sum);
    }
    trace_note!("Возвращаем успешный результат.");
    Ok(squared_deviation_sum / (values.len() - 1) as f64)
}
