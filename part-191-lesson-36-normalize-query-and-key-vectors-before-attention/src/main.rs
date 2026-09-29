// Урок 36.4. Нормализация векторов запроса и ключа перед вычислением внимания.
// Нормируем Q и K отдельно до сравнения, затем применяем позиционное вращение.

fn main() {
    let query_vector: Vec<f64> =
        part_188_lesson_36_normalize_token_vector_by_root_mean_square::normalize_vector_by_root_mean_square(
            &[2.0, 1.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let key_vector: Vec<f64> =
        part_188_lesson_36_normalize_token_vector_by_root_mean_square::normalize_vector_by_root_mean_square(
            &[1.0, 3.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let query_vector: [f64; 2] =
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
            [query_vector[0], query_vector[1]],
            2,
            0.1,
        );
    let key_vector: [f64; 2] =
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
            [key_vector[0], key_vector[1]],
            1,
            0.1,
        );
    let score: f64 =
        (query_vector[0] * key_vector[0] + query_vector[1] * key_vector[1]) / 2.0_f64.sqrt();
    assert!(score.is_finite());
    println!("QK-Norm + RoPE score={score:.4}");
}
