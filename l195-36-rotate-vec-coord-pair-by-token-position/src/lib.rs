//! Урок 195. Поворачиваем пары чисел, чтобы сравнение запроса и ключа учитывало их места в тексте.
//! Угол поворота равен номеру места, умноженному на заданный шаг угла в радианах.
//! cos задаёт вклад прежней координаты, sin — вклад другой координаты.
//! Поворот сохраняет длину стрелки. Если обе стрелки повернуть на одинаковый угол,
//! сумма произведений соответствующих координат не изменится.
//! Если углы разные, результат сравнения может измениться.

pub fn rotate_vec_coord_pair_by_token_position(
    vec: [f64; 2],
    position: usize,
    rotation_radians_per_token_position: f64,
) -> [f64; 2] {
    let rotation_angle_in_radians_from_position: f64 =
        position as f64 * rotation_radians_per_token_position;
    let (sin_as_cross_axis_rotation_weight, cos_as_same_axis_rotation_weight): (f64, f64) =
        rotation_angle_in_radians_from_position.sin_cos();
    [
        vec[0] * cos_as_same_axis_rotation_weight - vec[1] * sin_as_cross_axis_rotation_weight,
        vec[0] * sin_as_cross_axis_rotation_weight + vec[1] * cos_as_same_axis_rotation_weight,
    ]
}
#[cfg(test)]
mod tests {
    use lesson_float_comparison::check_f64_eq_1e_minus_10;

    #[test]
    /// Поворот сохраняет сумму квадратов координат, а значит и длину вектора.
    fn rotation_preserves_sum_of_squared_coords() {
        let input_value: [f64; 2] = [3.0, 4.0];
        let second_input_value: [f64; 2] =
            super::rotate_vec_coord_pair_by_token_position(input_value, 7, 0.1);
        assert!(check_f64_eq_1e_minus_10(
            second_input_value[0] * second_input_value[0]
                + second_input_value[1] * second_input_value[1],
            25.0
        ));
    }
}
