//! Расстояние между точками: корень из квадрата расстояния предыдущей части.

/// Евклидово расстояние: складываем квадраты разностей координат и извлекаем корень.
/// У каждой точки ровно две координаты; это задано типом массива.
use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

pub fn calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
    first_point: &[f64; 2],
    second_point: &[f64; 2],
) -> Result<f64, &'static str> {
    let squared_sum = calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
        first_point,
        second_point,
    )?;

    Ok(squared_sum.sqrt())
}
