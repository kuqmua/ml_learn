//! Урок 003. Длина вектора: квадратный корень из суммы квадратов координат.

/// Приближаем квадратный корень, многократно усредняя оценку и число, делённое на оценку.
/// Это метод Ньютона для небольших учебных входов.
/// Учебный аналог `f64::sqrt`: показывает шаги метода Ньютона и может работать медленнее.
/// Для отрицательного входа здесь panic, тогда как `sqrt` возвращает NaN.
/// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coords_then_add_results_as_unnormalized_alignment_where_pos_means_angle_below_90_degrees_neg_means_angle_above_90_degrees_and_0_means_perpendicular_or_zero_vec;

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
pub fn calc_vec_length_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer(
    vec: &[f64],
) -> f64 {
    approximate_square_root_by_repeated_averaging(
        multiply_matching_coords_then_add_results_as_unnormalized_alignment_where_pos_means_angle_below_90_degrees_neg_means_angle_above_90_degrees_and_0_means_perpendicular_or_zero_vec(vec, vec).expect(
            "внутренняя ошибка: сравнение вектора с самим собой не должно завершаться ошибкой",
        ),
    )
}
