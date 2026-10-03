//! Урок 001. Умножение соответствующих координат двух векторов и сложение результатов.

/// Умножаем соответствующие координаты и складываем результаты.
/// Скалярное произведение: умножаем соответствующие координаты двух векторов и складываем произведения.

/// Для ненулевых векторов: положительное значение — угол меньше 90° (включая 0°),
/// отрицательное — больше 90° (включая 180°), ноль — угол 90°.
/// Нулевой вектор тоже даёт ноль, но угол с ним не определён.
pub fn multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(
    first_vec: &[f64],
    second_vec: &[f64],
) -> Result<f64, &'static str> {
    if first_vec.len() != second_vec.len() {
        return Err("векторы должны иметь одинаковое число координат");
    }
    let mut sum_after_multiplying_matching_coords: f64 = 0.0;
    for index in 0..first_vec.len() {
        sum_after_multiplying_matching_coords += first_vec[index] * second_vec[index];
    }
    Ok(sum_after_multiplying_matching_coords)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `handles_perpendicular_and_mismatched_vecs` для этого примера.
    fn handles_perpendicular_and_mismatched_vecs() {
        assert_eq!(
            super::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&[1.0, 2.0], &[-2.0, 1.0]),
            Ok(0.0)
        );
        assert!(
            super::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&[1.0], &[1.0, 2.0]).is_err()
        );
    }
}
