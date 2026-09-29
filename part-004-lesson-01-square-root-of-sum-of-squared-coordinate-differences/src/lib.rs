//! Урок 004. Квадратный корень из суммы квадратов разностей координат двух точек.

/// Сумма квадратов покоординатных разностей.
/// Квадрат евклидова расстояния: вычитаем соответствующие координаты, возводим разности в квадрат и складываем.
pub fn calculate_squared_point_distance_by_summing_squared_coordinate_differences(
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
    lesson_trace::trace_step!(squared_sum);
    // Повторяем расчёт для каждого элемента последовательности.
    for index in 0..left.len() {
        lesson_trace::trace_step!(index);
        // Сохраняем результат этого шага в `difference`.
        let difference: f64 = left[index] - right[index];
        lesson_trace::trace_step!(difference);
        // Обновляем значение результатом текущего вычисления.
        squared_sum += difference * difference;
        lesson_trace::trace_step!(squared_sum);
    }
    // Возвращаем успешный результат.
    Ok(squared_sum)
}

/// Расстояние — длина вектора разностей; используем норму из урока 01.3.
/// Евклидово расстояние: берём корень из суммы квадратов разностей соответствующих координат.
pub fn calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
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
    lesson_trace::trace_step!(differences);
    // Возвращаем успешный результат.
    Ok(
        part_003_lesson_01_square_root_of_sum_of_squared_vector_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(
            &differences,
        ),
    )
}
