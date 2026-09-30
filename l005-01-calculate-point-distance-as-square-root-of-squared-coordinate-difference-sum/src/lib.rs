//! Расстояние между точками: корень из квадрата расстояния предыдущей части.

/// Евклидово расстояние: складываем квадраты разностей координат и извлекаем корень.
use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

use lesson_trace::{trace_note, trace_step};

pub fn calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    let squared_sum =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(left, right)?;
    trace_step!(squared_sum);
    trace_note!(
        "Суммирование показано в предыдущей части; здесь используем стандартный квадратный корень."
    );
    let distance = squared_sum.sqrt();
    trace_step!(distance);
    Ok(distance)
}
