// Урок 36.2. Вращение пар координат запроса и ключа по позиции токена.
// Позиция вращает пары координат Q и K, сохраняя их длину.

fn calculate_scalar_product_of_two_vectors(first_value: [f64; 2], second_value: [f64; 2]) -> f64 {
    first_value[0] * second_value[0] + first_value[1] * second_value[1]
}
fn main() {
    let query_vector = [1.0, 0.0];
    let key_vector = [1.0, 0.0];
    let same = calculate_scalar_product_of_two_vectors(
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
            query_vector,
            3,
            0.2,
        ),
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
            key_vector, 3, 0.2,
        ),
    );
    let distant = calculate_scalar_product_of_two_vectors(
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
            query_vector,
            3,
            0.2,
        ),
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
            key_vector, 8, 0.2,
        ),
    );
    assert!((same - 1.0).abs() < 1e-12);
    assert!(distant < same);
    println!("одинаковая позиция: {same:.3}; разные позиции: {distant:.3}");
    visualize_rotate_query_and_key_coordinate_pairs_by_position();
}

fn visualize_rotate_query_and_key_coordinate_pairs_by_position() {
    let query_vector = [1.0, 0.0];
    let query_vector =
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
            query_vector,
            0,
            0.2,
        );
    let points: Vec<_> = (0..=20)
        .map(|position_index| {
            let key_vector =
                part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
                    [1.0, 0.0],
                    position_index,
                    0.2,
                );
            (
                position_index as f64,
                calculate_scalar_product_of_two_vectors(query_vector, key_vector),
            )
        })
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "rope-relative",
        "RoPE и расстояние между позициями",
        "сдвиг позиции",
        "скалярное произведение",
        &[lesson_visualization::Series {
            name: "score",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
