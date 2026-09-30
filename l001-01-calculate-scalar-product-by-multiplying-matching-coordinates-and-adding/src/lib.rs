//! Урок 001. Скалярное произведение: умножение соответствующих координат двух векторов и сложение произведений.

/// Умножаем соответствующие координаты и складываем результаты.
/// Скалярное произведение: умножаем соответствующие координаты двух векторов и складываем произведения.

pub fn calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
    left: &[f64],

    right: &[f64],
) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("векторы должны быть одинаковой длины");
    }
    let mut sum: f64 = 0.0;
    for index in 0..left.len() {
        sum += left[index] * right[index];
    }
    Ok(sum)
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
            super::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                &[1.0, 2.0],
                &[-2.0, 1.0]
            ),
            Ok(0.0)
        );
        assert!(
            super::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                &[1.0],
                &[1.0, 2.0]
            )
            .is_err()
        );
    }
}
