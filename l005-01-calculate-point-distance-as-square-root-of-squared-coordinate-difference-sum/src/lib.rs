//! Расстояние между точками: корень из квадрата расстояния предыдущей части.

/// Евклидово расстояние: складываем квадраты разностей координат и извлекаем корень.
/// У каждой точки ровно две координаты; это задано типом массива.
use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

pub fn calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
    left: &[f64; 2],
    right: &[f64; 2],
) -> Result<f64, &'static str> {
    let squared_sum =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(left, right)?;
    let distance = squared_sum.sqrt();
    Ok(distance)
}
