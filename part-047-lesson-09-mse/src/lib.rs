//! Вычисления и примеры урока part-047-lesson-09-mse.

fn validate_prediction_pairs(targets: &[f64], predictions: &[f64]) -> Result<(), &'static str> {
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
pub fn mean_squared_error(targets: &[f64], predictions: &[f64]) -> Result<f64, &'static str> {
    // Используем подготовленное значение в следующем шаге примера.
    validate_prediction_pairs(targets, predictions)?;
    // Сохраняем результат этого шага в `squared_sum`.
    let mut squared_sum = 0.0;
    // Повторяем расчёт для каждого элемента последовательности.
    for index in 0..targets.len() {
        // Сохраняем результат этого шага в `error`.
        let error = predictions[index] - targets[index];
        // Обновляем значение результатом текущего вычисления.
        squared_sum += error * error;
    }
    // Возвращаем успешный результат.
    Ok(squared_sum / targets.len() as f64)
}
