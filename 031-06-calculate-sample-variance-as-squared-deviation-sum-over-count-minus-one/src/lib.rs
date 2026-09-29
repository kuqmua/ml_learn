//! Урок 031. Разброс значений (выборочная дисперсия): сумма квадратов отклонений от среднего, делённая на число значений минус один.

/// Выборочная дисперсия использует среднее из урока 06.1.
/// Выборочная дисперсия: сумму квадратов отклонений от среднего делим на (число значений − 1).
pub fn calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(
    values: &[f64],
) -> Result<f64, &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if values.len() < 2 {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    // Сохраняем результат этого шага в `average`.
    let average: f64 =
        l029_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(values)?;
    lesson_trace::trace_step!(average);
    // Сохраняем результат этого шага в `squared_deviation_sum`.
    let mut squared_deviation_sum: f64 = 0.0;
    lesson_trace::trace_step!(squared_deviation_sum);
    // Повторяем расчёт для каждого элемента последовательности.
    for &value in values {
        lesson_trace::trace_step!(value);
        // Сохраняем результат этого шага в `deviation`.
        let deviation: f64 = value - average;
        lesson_trace::trace_step!(deviation);
        // Обновляем значение результатом текущего вычисления.
        squared_deviation_sum += deviation * deviation;
        lesson_trace::trace_step!(squared_deviation_sum);
    }
    // Возвращаем успешный результат.
    Ok(squared_deviation_sum / (values.len() - 1) as f64)
}
