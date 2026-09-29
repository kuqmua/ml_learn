//! Вычисления и примеры урока part-004-lesson-01-euclidean-distance-between-two-points.

/// Сумма квадратов покоординатных разностей.
pub fn squared_euclidean_distance_between_two_points(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if left.len() != right.len() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("точки должны иметь одинаковое число координат");
    }
    // Сохраняем результат этого шага в `squared_sum`.
    let mut squared_sum: f64 = 0.0;
    // Повторяем расчёт для каждого элемента последовательности.
    for index in 0..left.len() {
        // Сохраняем результат этого шага в `difference`.
        let difference: f64 = left[index] - right[index];
        // Обновляем значение результатом текущего вычисления.
        squared_sum += difference * difference;
    }
    // Возвращаем успешный результат.
    Ok(squared_sum)
}

/// Расстояние — длина вектора разностей; используем норму из урока 01.3.
pub fn euclidean_distance_between_two_points(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if left.len() != right.len() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("точки должны иметь одинаковое число координат");
    }
    // Собираем значения для `differences` в коллекцию.
    let differences: Vec<f64> = left
        .iter()
        .zip(right)
        .map(|(&first_value, &second_value)| first_value - second_value)
        .collect();
    // Возвращаем успешный результат.
    Ok(
        part_003_lesson_01_calculate_euclidean_length_of_one_vector::euclidean_norm_of_vector(
            &differences,
        ),
    )
}
