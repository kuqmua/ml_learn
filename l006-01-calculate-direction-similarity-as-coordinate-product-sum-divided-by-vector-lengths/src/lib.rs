//! Урок 006. Сходство направлений векторов: сумма произведений координат, делённая на произведение длин.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
/// Косинусное сходство: сумму произведений соответствующих координат делим на произведение длин векторов.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates;

pub fn calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    let numerator: f64 = multiply_matching_coordinates_then_add_results(left, right)?;
    let denominator: f64 =
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(left)
            * calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(right);
    if denominator == 0.0 {
        return Err("у нулевого вектора нет направления");
    }
    Ok(numerator / denominator)
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
            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(

            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[-1.0, 0.0]),

            Ok(-1.0)
        );
        assert!(
            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[0.0, 0.0])
                .is_err()
        );
    }
}
