//! Урок 003. Считаем длину стрелки от начала координат до заданной точки.
//! Каждое число умножаем само на себя, складываем результаты и берём квадратный корень.
//! Для [3, 4]: 3×3 + 4×4 = 25, корень из 25 равен 5, потому что 5×5 = 25.
//! Смена знака координаты меняет направление, но не длину.

use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

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
pub fn calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer(
    vec: &[f64],
) -> f64 {
    approximate_square_root_by_repeated_averaging(
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(vec, vec).expect(
            "внутренняя ошибка: сравнение вектора с самим собой не должно завершаться ошибкой",
        ),
    )
}
