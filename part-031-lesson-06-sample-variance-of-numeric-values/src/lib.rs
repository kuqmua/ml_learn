//! Вычисления и примеры урока part-031-lesson-06-sample-variance-of-numeric-values.

/// Выборочная дисперсия использует среднее из урока 06.1.
pub fn sample_variance_of_numeric_values(values: &[f64]) -> Result<f64, &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if values.len() < 2 {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    // Сохраняем результат этого шага в `average`.
    let average: f64 =
        part_029_lesson_06_arithmetic_mean_of_numeric_values::arithmetic_mean_of_numeric_values(
            values,
        )?;
    // Сохраняем результат этого шага в `squared_deviation_sum`.
    let mut squared_deviation_sum: f64 = 0.0;
    // Повторяем расчёт для каждого элемента последовательности.
    for &value in values {
        // Сохраняем результат этого шага в `deviation`.
        let deviation: f64 = value - average;
        // Обновляем значение результатом текущего вычисления.
        squared_deviation_sum += deviation * deviation;
    }
    // Возвращаем успешный результат.
    Ok(squared_deviation_sum / (values.len() - 1) as f64)
}
