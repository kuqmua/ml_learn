//! Урок 002. Длина пути вдоль осей (норма L1): сложение модулей координат вектора.

/// Получаем норму L1: складываем модули координат — длины перемещений вдоль каждой оси.
/// Для [3, 4] это 7; обычная длина прямого отрезка (норма L2) равна 5.

pub fn calc_sum_of_absolute_vec_coords_as_total_axis_aligned_len_where_0_means_zero_vec(
    vec: &[f64],
) -> f64 {
    let mut sum_of_absolute_coords: f64 = 0.0;
    for &coord in vec {
        sum_of_absolute_coords += if coord < 0.0 { -coord } else { coord };
    }
    sum_of_absolute_coords
}
