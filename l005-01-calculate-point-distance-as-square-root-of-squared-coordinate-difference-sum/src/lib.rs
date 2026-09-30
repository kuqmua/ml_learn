//! Расстояние между точками: корень из квадрата расстояния предыдущей части.

/// Евклидово расстояние: складываем квадраты разностей координат и извлекаем корень.
/// Оба массива имеют одинаковую длину `N`; число координат проверяет компилятор.
use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

pub fn calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences<
    const N: usize,
>(
    left: &[f64; N],
    right: &[f64; N],
) -> Result<f64, &'static str> {
    let squared_sum =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(left, right)?;
    let distance = squared_sum.sqrt();
    Ok(distance)
}
