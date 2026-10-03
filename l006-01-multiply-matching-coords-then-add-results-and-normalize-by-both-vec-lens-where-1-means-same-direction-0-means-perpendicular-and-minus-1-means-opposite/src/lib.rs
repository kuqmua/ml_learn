//! Урок 006. cos угла между векторами: умножение соответствующих координат, сложение и деление на длины.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
/// Косинусное сходство: сумму произведений соответствующих координат делим на произведение длин векторов.
use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;
use l003_01_calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer::calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer;

pub fn multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens_where_1_means_same_direction_0_means_perpendicular_and_minus_1_means_opposite(
    first_vec: &[f64],
    second_vec: &[f64],
) -> Result<f64, &'static str> {
    let sum_after_multiplying_coords: f64 =
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(first_vec, second_vec)?;
    let multiplied_vec_lens: f64 =
        calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer(first_vec)
            * calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer(second_vec);
    if multiplied_vec_lens == 0.0 {
        return Err("у нулевого вектора нет направления");
    }
    Ok(sum_after_multiplying_coords / multiplied_vec_lens)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `reuses_earlier_lessons_and_rejects_zero_vec` для этого примера.
    fn reuses_earlier_lessons_and_rejects_zero_vec() {
        assert_eq!(
            super::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens_where_1_means_same_direction_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            super::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens_where_1_means_same_direction_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[-1.0, 0.0]),
            Ok(-1.0)
        );
        assert!(super::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens_where_1_means_same_direction_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[0.0, 0.0]).is_err());
    }
}
