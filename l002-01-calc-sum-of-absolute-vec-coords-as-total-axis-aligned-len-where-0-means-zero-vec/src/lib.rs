//! Урок 002. Складываем величины чисел, не учитывая их знаки.
//! Для [-3, 4] получаем 3 + 4 = 7. Минус не уменьшает результат.
//! Это длина пути, если двигаться только вдоль осей: сначала на 3, затем на 4.
//! Длина прямого пути считается иначе — в следующем уроке.

pub fn calc_sum_of_absolute_vec_coords_as_total_axis_aligned_len_where_0_means_zero_vec(
    vec: &[f64],
) -> f64 {
    let mut sum_of_absolute_coords: f64 = 0.0;
    for &coord in vec {
        sum_of_absolute_coords += if coord < 0.0 { -coord } else { coord };
    }
    sum_of_absolute_coords
}
