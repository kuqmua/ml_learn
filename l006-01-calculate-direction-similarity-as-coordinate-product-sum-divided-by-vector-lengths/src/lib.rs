//! Урок 006. Сходство направлений векторов: сумма произведений координат, делённая на произведение длин.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
/// Косинусное сходство: сумму произведений соответствующих координат делим на произведение длин векторов.
use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates;

use lesson_trace::{trace_note, trace_step};

pub fn calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    trace_note!("Сохраняем результат этого шага в `numerator`.");
    let numerator: f64 =
        calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(left, right)?;
    trace_step!(numerator);
    trace_note!("Сохраняем результат этого шага в `denominator`.");
    let denominator: f64 =
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(left)
            * calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(right);
    trace_step!(denominator);
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if denominator == 0.0 {
        trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("у нулевого вектора нет направления");
    }
    trace_note!("Возвращаем успешный результат.");
    Ok(numerator / denominator)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {
    use lesson_trace::trace_note;

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `reuses_earlier_lessons_and_rejects_zero_vector` для этого примера.
    fn reuses_earlier_lessons_and_rejects_zero_vector() {
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(
            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[0.0, 1.0]),
            Ok(0.0)
        );
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        trace_note!("Задаём именованное поле или параметр.");
        trace_note!("Возвращаем успешный результат.");
        assert_eq!(

            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[-1.0, 0.0]),

            Ok(-1.0)
        );
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(
            super::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&[1.0, 0.0], &[0.0, 0.0])
                .is_err()
        );
    }
}
