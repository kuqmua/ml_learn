//! Урок 006. cos угла между векторами: умножение соответствующих координат, сложение и деление на длины.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
/// Косинусное сходство: сумму произведений соответствующих координат делим на произведение длин векторов.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer;

pub fn calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(
    first_vector: &[f64],
    second_vector: &[f64],
) -> Result<f64, &'static str> {
    let sum_after_multiplying_coordinates: f64 =
        multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(first_vector, second_vector)?;
    let multiplied_vector_lengths: f64 =
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer(first_vector)
            * calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer(second_vector);
    if multiplied_vector_lengths == 0.0 {
        return Err("у нулевого вектора нет направления");
    }
    Ok(sum_after_multiplying_coordinates / multiplied_vector_lengths)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `reuses_earlier_lessons_and_rejects_zero_vector` для этого примера.
    fn reuses_earlier_lessons_and_rejects_zero_vector() {
        assert_eq!(
            super::calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            super::calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[-1.0, 0.0]),
            Ok(-1.0)
        );
        assert!(super::calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[0.0, 0.0]).is_err());
    }
}
