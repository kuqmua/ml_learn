// Урок 36.4. QK-Norm перед attention.
// Нормируем Q и K отдельно до сравнения, затем применяем позиционное вращение.

fn main() {
    let query_vector =
        part_188_lesson_36_root_mean_square_normalization::root_mean_square_normalization(
            &[2.0, 1.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let key_vector =
        part_188_lesson_36_root_mean_square_normalization::root_mean_square_normalization(
            &[1.0, 3.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let query_vector =
        part_189_lesson_36_rotary_position_embedding::rotate_coordinate_pair_by_position(
            [query_vector[0], query_vector[1]],
            2,
            0.1,
        );
    let key_vector =
        part_189_lesson_36_rotary_position_embedding::rotate_coordinate_pair_by_position(
            [key_vector[0], key_vector[1]],
            1,
            0.1,
        );
    let score =
        (query_vector[0] * key_vector[0] + query_vector[1] * key_vector[1]) / 2.0_f64.sqrt();
    assert!(score.is_finite());
    println!("QK-Norm + RoPE score={score:.4}");
}
