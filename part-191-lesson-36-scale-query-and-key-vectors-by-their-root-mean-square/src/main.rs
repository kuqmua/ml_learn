// Урок 36.4. Масштабирование запросов и ключей по корню из среднего квадрата координат.
// Связь с принятой терминологией: Нормализация векторов запроса и ключа перед вычислением внимания.
// Зачем здесь эта тема: После позиционного вращения масштаб Q и K может влиять на резкость оценок.
// Почему код устроен так: Нормируем оба перед скалярным произведением, чтобы сравнение зависело от
//   направления.
// Представь: Два одинаково направленных ключа разного масштаба после нормировки сравниваются ближе
//   по смыслу направления.
// Нормируем Q и K отдельно до сравнения, затем применяем позиционное вращение.

fn main() {
    lesson_trace::enable();
    // ε=10⁻⁶ добавляется к среднему квадрату координат Q и K, чтобы RMSNorm был определён и для нуля.
    let query_vector: Vec<f64> =
        part_188_lesson_36_divide_coordinates_by_square_root_of_mean_square_and_apply_weights::divide_coordinates_by_root_mean_square_then_apply_weights(
            &[2.0, 1.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    lesson_trace::trace_step!(query_vector);
    let key_vector: Vec<f64> =
        part_188_lesson_36_divide_coordinates_by_square_root_of_mean_square_and_apply_weights::divide_coordinates_by_root_mean_square_then_apply_weights(
            &[1.0, 3.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    lesson_trace::trace_step!(key_vector);
    let query_vector: [f64; 2] =
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
            [query_vector[0], query_vector[1]],
            2,
            0.1,
        );
    lesson_trace::trace_step!(query_vector);
    let key_vector: [f64; 2] =
        part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
            [key_vector[0], key_vector[1]],
            1,
            0.1,
        );
    lesson_trace::trace_step!(key_vector);
    let score: f64 =
        (query_vector[0] * key_vector[0] + query_vector[1] * key_vector[1]) / 2.0_f64.sqrt();
    lesson_trace::trace_step!(score);
    assert!(score.is_finite());
    println!("QK-Norm + RoPE score={score:.4}");
}
