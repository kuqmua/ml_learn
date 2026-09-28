//! Вычисления и примеры урока part-031-lesson-06-variance.

/// Выборочная дисперсия использует среднее из урока 06.1.
pub fn sample_variance(values: &[f64]) -> Result<f64, &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if values.len() < 2 {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    // Сохраняем результат этого шага в `average`.
    let average = lesson_029::mean(values)?;
    // Сохраняем результат этого шага в `squared_deviation_sum`.
    let mut squared_deviation_sum = 0.0;
    // Повторяем расчёт для каждого элемента последовательности.
    for &value in values {
        // Сохраняем результат этого шага в `deviation`.
        let deviation = value - average;
        // Обновляем значение результатом текущего вычисления.
        squared_deviation_sum += deviation * deviation;
    }
    // Возвращаем успешный результат.
    Ok(squared_deviation_sum / (values.len() - 1) as f64)
}
