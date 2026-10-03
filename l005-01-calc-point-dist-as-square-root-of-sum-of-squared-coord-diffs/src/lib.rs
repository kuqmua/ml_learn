//! Урок 005. Считаем расстояние между двумя точками по прямой.
//! Берём сумму квадратов разниц из предыдущего урока и извлекаем квадратный корень.
//! Например, для разниц 3 и 4 получаем корень из 25, то есть расстояние 5.
//! Число координат у обеих точек должно совпадать; это проверяет тип массива.
//! Бесконечные и неопределённые координаты, а также переполнение дают ошибку.

use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs::calc_squared_point_dist_by_summing_squared_coord_diffs;

pub fn calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs(
    first_point: &[f64; 2],
    second_point: &[f64; 2],
) -> Result<f64, &'static str> {
    let squared_sum =
        calc_squared_point_dist_by_summing_squared_coord_diffs(first_point, second_point)?;

    Ok(squared_sum.sqrt())
}
