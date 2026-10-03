// Урок 36.2. Учёт позиции в тексте: поворот пар координат запроса и ключа.
// Связь с принятой терминологией: Вращение пар координат запроса и ключа по позиции токена.
// Зачем здесь эта тема: Положение токена должно влиять на сравнение Q и K без отдельного сложения
//   позиционного вектора.
// Почему код устроен так: Поворачиваем пары координат на угол позиции и проверяем сохранение длины.
// Представь: Поворачиваем координаты по номеру позиции, сохраняя длину, но меняя сравнение разных
//   мест.
// Позиция вращает пары координат Q и K, сохраняя их длину.

/// Скалярное произведение: умножаем соответствующие координаты двух векторов и складываем произведения.
use l195_36_encode_text_position_by_rotating_query_and_key_coordinate_pairs::rotate_vector_coordinate_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vector_length;

fn multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(
    first_value: [f64; 2],
    second_value: [f64; 2],
) -> f64 {
    first_value[0] * second_value[0] + first_value[1] * second_value[1]
}
fn main() {
    let query_vector: [f64; 2] = [1.0, 0.0];
    let key_vector: [f64; 2] = [1.0, 0.0];
    let query_key_match_after_equal_position_rotation: f64 = multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(
        rotate_vector_coordinate_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vector_length(query_vector, 3, 0.2),
        rotate_vector_coordinate_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vector_length(key_vector, 3, 0.2),
    );

    assert!((query_key_match_after_equal_position_rotation - 1.0).abs() < 1e-12);
    assert!(
        multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(
            rotate_vector_coordinate_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vector_length(query_vector, 3, 0.2),
            rotate_vector_coordinate_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vector_length(key_vector, 8, 0.2),
        ) < query_key_match_after_equal_position_rotation
    );

    plot_sum_after_multiplying_rotated_coordinates_for_relative_positions();
}

fn plot_sum_after_multiplying_rotated_coordinates_for_relative_positions() {
    let query_vector: [f64; 2] = [1.0, 0.0];
    let query_vector: [f64; 2] =
        rotate_vector_coordinate_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vector_length(query_vector, 0, 0.2);

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
                        multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(
                            query_vector,
                            rotate_vector_coordinate_pair_by_token_position_to_encode_relative_position_in_query_key_matches_while_preserving_vector_length(
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
