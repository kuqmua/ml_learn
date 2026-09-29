//! Урок 005. Сходство направлений векторов: сумма произведений координат, делённая на произведение длин.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
/// Косинусное сходство: сумму произведений соответствующих координат делим на произведение длин векторов.
pub fn calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    // Сохраняем результат этого шага в `numerator`.
    let numerator: f64 =
        part_001_lesson_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
            left, right,
        )?;
    lesson_trace::trace_step!(numerator);
    // Сохраняем результат этого шага в `denominator`.
    let denominator: f64 =
        part_003_lesson_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(left)
            * part_003_lesson_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(
                right,
            );
    lesson_trace::trace_step!(denominator);
    // Выбираем дальнейший шаг по выполнению условия.
    if denominator == 0.0 {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("у нулевого вектора нет направления");
    }
    // Возвращаем успешный результат.
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
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(
            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[0.0, 1.0]),
            Ok(0.0)
        );
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(
            // Задаём именованное поле или параметр.
            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[-1.0, 0.0]),
            // Возвращаем успешный результат.
            Ok(-1.0)
        );
        // Проверяем ожидаемое свойство учебного примера.
        assert!(
            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[0.0, 0.0])
                .is_err()
        );
    }
}
