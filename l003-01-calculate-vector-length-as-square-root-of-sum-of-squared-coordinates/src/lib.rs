//! Урок 003. Длина вектора: квадратный корень из суммы квадратов координат.

/// Приближаем квадратный корень, многократно усредняя оценку и число, делённое на оценку.
/// Это метод Ньютона для небольших учебных входов.
/// Учебный аналог `f64::sqrt`: показывает шаги метода Ньютона и может работать медленнее.
/// Для отрицательного входа здесь panic, тогда как `sqrt` возвращает NaN.
/// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector;

fn approximate_square_root_by_repeated_averaging(value: f64) -> f64 {
    assert!(value >= 0.0, "корень из отрицательного числа");
    if value == 0.0 {
        return 0.0;
    }
    let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
    for _ in 0..80 {
        estimate = (estimate + value / estimate) / 2.0;
    }
    estimate
}

/// Берём квадратный корень из суммы квадратов координат: sqrt(v₁² + v₂² + …).
/// Получаем длину вектора — в математике это евклидова норма (норма L2).
/// По теореме Пифагора это расстояние от начала координат до конца вектора.
/// Например, для [3, 4]: sqrt(9 + 16) = 5.
pub fn calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer(
    vector: &[f64],
) -> f64 {
    approximate_square_root_by_repeated_averaging(
        multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(vector, vector).expect(
            "внутренняя ошибка: сравнение вектора с самим собой не должно завершаться ошибкой",
        ),
    )
}
