//! Урок 001. Умножение соответствующих координат двух векторов и сложение результатов.

/// Умножаем соответствующие координаты и складываем результаты.
/// Скалярное произведение: умножаем соответствующие координаты двух векторов и складываем произведения.

pub fn multiply_matching_coordinates_then_add_results(
    first_vector: &[f64],
    second_vector: &[f64],
) -> Result<f64, &'static str> {
    if first_vector.len() != second_vector.len() {
        return Err("векторы должны быть одинаковой длины");
    }
    let mut sum_after_multiplying_matching_coordinates: f64 = 0.0;
    for index in 0..first_vector.len() {
        sum_after_multiplying_matching_coordinates += first_vector[index] * second_vector[index];
    }
    Ok(sum_after_multiplying_matching_coordinates)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `handles_perpendicular_and_mismatched_vectors` для этого примера.
    fn handles_perpendicular_and_mismatched_vectors() {
        assert_eq!(
            super::multiply_matching_coordinates_then_add_results(&[1.0, 2.0], &[-2.0, 1.0]),
            Ok(0.0)
        );
        assert!(
            super::multiply_matching_coordinates_then_add_results(&[1.0], &[1.0, 2.0]).is_err()
        );
    }
}
