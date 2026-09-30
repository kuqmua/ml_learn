//! Расстояние между точками: корень из квадрата расстояния предыдущей части.

/// Евклидово расстояние: складываем квадраты разностей координат и извлекаем корень.
pub fn calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    let squared_sum = l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(left, right)?;
    lesson_trace::trace_step!(squared_sum);
    // Суммирование показано в предыдущей части; здесь используем стандартный квадратный корень.
    let distance = squared_sum.sqrt();
    lesson_trace::trace_step!(distance);
    Ok(distance)
}
