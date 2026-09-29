// Урок 36.2. Поворот пар координат запроса и ключа в зависимости от позиции в тексте.
// Связь с принятой терминологией: Вращение пар координат запроса и ключа по позиции токена.
// Зачем здесь эта тема: Положение токена должно влиять на сравнение Q и K без отдельного сложения
//   позиционного вектора.
// Почему код устроен так: Поворачиваем пары координат на угол позиции и проверяем сохранение длины.
// Представь: Поворачиваем координаты по номеру позиции, сохраняя длину, но меняя сравнение разных
//   мест.
// Позиция вращает пары координат Q и K, сохраняя их длину.

/// Скалярное произведение: умножаем соответствующие координаты двух векторов и складываем произведения.
fn multiply_matching_coordinates_of_two_vectors_then_add(
    first_value: [f64; 2],
    second_value: [f64; 2],
) -> f64 {
    first_value[0] * second_value[0] + first_value[1] * second_value[1]
}
fn main() {
    lesson_trace::enable();
    let query_vector: [f64; 2] = [1.0, 0.0];
    lesson_trace::trace_step!(query_vector);
    let key_vector: [f64; 2] = [1.0, 0.0];
    lesson_trace::trace_step!(key_vector);
    let same: f64 = multiply_matching_coordinates_of_two_vectors_then_add(
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
            query_vector,
            3,
            0.2,
        ),
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
            key_vector, 3, 0.2,
        ),
    );
    lesson_trace::trace_step!(same);
    let distant: f64 = multiply_matching_coordinates_of_two_vectors_then_add(
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
            query_vector,
            3,
            0.2,
        ),
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
            key_vector, 8, 0.2,
        ),
    );
    lesson_trace::trace_step!(distant);
    assert!((same - 1.0).abs() < 1e-12);
    assert!(distant < same);
    println!("одинаковая позиция: {same:.3}; разные позиции: {distant:.3}");
    lesson_trace::disable();
    plot_coordinate_product_sum_for_relative_position_rotations();
}

fn plot_coordinate_product_sum_for_relative_position_rotations() {
    let query_vector: [f64; 2] = [1.0, 0.0];
    let query_vector: [f64; 2] =
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
            query_vector,
            0,
            0.2,
        );
    let points: Vec<(f64, f64)> = (0..=20)
        .map(|position_index| {
            let key_vector: [f64; 2] =
                part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
                    [1.0, 0.0],
                    position_index,
                    0.2,
                );
            (
                position_index as f64,
                multiply_matching_coordinates_of_two_vectors_then_add(query_vector, key_vector),
            )
        })
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
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
