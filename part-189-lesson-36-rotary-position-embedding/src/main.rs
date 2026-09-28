// Урок 36.2. Вращательные позиционные признаки RoPE.
// Позиция вращает пары координат Q и K, сохраняя их длину.

use part_189_lesson_36_rotary_position_embedding::rotate_coordinate_pair_by_position;
fn calculate_scalar_product_of_two_vectors(first_value: [f64; 2], second_value: [f64; 2]) -> f64 {
    first_value[0] * second_value[0] + first_value[1] * second_value[1]
}
fn main() {
    let query_vector = [1.0, 0.0];
    let key_vector = [1.0, 0.0];
    let same = calculate_scalar_product_of_two_vectors(
        rotate_coordinate_pair_by_position(query_vector, 3, 0.2),
        rotate_coordinate_pair_by_position(key_vector, 3, 0.2),
    );
    let distant = calculate_scalar_product_of_two_vectors(
        rotate_coordinate_pair_by_position(query_vector, 3, 0.2),
        rotate_coordinate_pair_by_position(key_vector, 8, 0.2),
    );
    assert!((same - 1.0).abs() < 1e-12);
    assert!(distant < same);
    println!("одинаковая позиция: {same:.3}; разные позиции: {distant:.3}");
    visualize();
}

fn visualize() {
    let query_vector = [1.0, 0.0];
    let query_vector = rotate_coordinate_pair_by_position(query_vector, 0, 0.2);
    let points: Vec<_> = (0..=20)
        .map(|position_index| {
            let key_vector = rotate_coordinate_pair_by_position([1.0, 0.0], position_index, 0.2);
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
