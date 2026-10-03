// Урок 36.2. Учёт позиции в тексте: поворот пар координат запроса и ключа.
// Связь с принятой терминологией: Вращение пар координат запроса и ключа по позиции токена.
// Зачем здесь эта тема: Положение токена должно влиять на сравнение Q и K без отдельного сложения
//   позиционного вектора.
// Почему код устроен так: Поворачиваем пары координат на угол позиции и проверяем сохранение длины.
// Представь: Поворачиваем координаты по номеру позиции, сохраняя длину, но меняя сравнение разных
//   мест.
// Позиция вращает пары координат Q и K, сохраняя их длину.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

/// Скалярное произведение: умножаем соответствующие координаты двух векторов и складываем произведения.
use l195_36_rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len::rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len;

fn multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(
    first_value: [f64; 2],
    second_value: [f64; 2],
) -> f64 {
    first_value[0] * second_value[0] + first_value[1] * second_value[1]
}
fn main() {
    let query_vec: [f64; 2] = [1.0, 0.0];
    let key_vec: [f64; 2] = [1.0, 0.0];
    let query_key_match_after_equal_position_rotation: f64 = multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(
        rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len(query_vec, 3, 0.2),
        rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len(key_vec, 3, 0.2),
    );

    assert!(check_f64_eq_1e_minus_12(
        query_key_match_after_equal_position_rotation,
        1.0
    ));
    assert!(
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(
            rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len(query_vec, 3, 0.2),
            rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len(key_vec, 8, 0.2),
        ) < query_key_match_after_equal_position_rotation
    );

    plot_sum_after_multiplying_rotated_coords_for_relative_positions();
}

fn plot_sum_after_multiplying_rotated_coords_for_relative_positions() {
    let query_vec: [f64; 2] = [1.0, 0.0];
    let query_vec: [f64; 2] =
        rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len(query_vec, 0, 0.2);

    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "rope-relative",
        "RoPE и расстояние между позициями",
        "сдвиг позиции",
        "скалярное произведение",
        &[lesson_visualization::Series {
            name: "score",
            points: &(0..=20)
                .map(|position_index| {
                    (
                        position_index as f64,
                        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(
                            query_vec,
                            rotate_vec_coord_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vec_len(
                                [1.0, 0.0],
                                position_index,
                                0.2,
                            ),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
