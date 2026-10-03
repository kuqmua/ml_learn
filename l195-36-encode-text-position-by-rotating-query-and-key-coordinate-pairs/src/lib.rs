//! Урок 195. Учёт позиции в тексте: поворот пар координат запроса и ключа.
//! Связь с принятой терминологией: Вращательные позиционные признаки RoPE.

/// Вращаем пару координат согласно позиции; для многомерной головы это делают по парам.

pub fn rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_length(
    vec: [f64; 2],
    position: usize,
    rotation_radians_per_token_position: f64,
) -> [f64; 2] {
    let rotation_angle_in_radians_from_position: f64 =
        position as f64 * rotation_radians_per_token_position;
    let (
        sin_as_cross_axis_rotation_weight_where_0_means_none_1_means_full_and_minus_1_means_reversed_contribution,
        cos_as_same_axis_rotation_weight_where_1_keeps_0_removes_and_minus_1_reverses_contribution,
    ): (f64, f64) = rotation_angle_in_radians_from_position.sin_cos();
    [
        vec[0] * cos_as_same_axis_rotation_weight_where_1_keeps_0_removes_and_minus_1_reverses_contribution - vec[1] * sin_as_cross_axis_rotation_weight_where_0_means_none_1_means_full_and_minus_1_means_reversed_contribution,
        vec[0] * sin_as_cross_axis_rotation_weight_where_0_means_none_1_means_full_and_minus_1_means_reversed_contribution + vec[1] * cos_as_same_axis_rotation_weight_where_1_keeps_0_removes_and_minus_1_reverses_contribution,
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
            super::rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_length(input_value, 7, 0.1);
        assert!(check_f64_eq_1e_minus_10(
            second_input_value[0] * second_input_value[0]
                + second_input_value[1] * second_input_value[1],
            25.0
        ));
    }
}
