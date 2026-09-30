// Урок 36.4. Масштабирование запросов и ключей по корню из среднего квадрата координат.
// Связь с принятой терминологией: Нормализация векторов запроса и ключа перед вычислением внимания.
// Зачем здесь эта тема: После позиционного вращения масштаб Q и K может влиять на резкость оценок.
// Почему код устроен так: Нормируем оба перед скалярным произведением, чтобы сравнение зависело от
//   направления.
// Представь: Два одинаково направленных ключа разного масштаба после нормировки сравниваются ближе
//   по смыслу направления.
// Нормируем Q и K отдельно до сравнения, затем применяем позиционное вращение.

use l194_36_normalize_vector_scale_by_dividing_by_root_mean_square_and_applying_weights::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights;
use l195_36_encode_text_position_by_rotating_query_and_key_coordinate_pairs::rotate_vector_coordinate_pair_by_token_position;

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!(
        "ε=10⁻⁶ добавляется к среднему квадрату координат Q и K, чтобы RMSNorm был определён и для нуля."
    );
    let query_vector: Vec<f64> =
        normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
            &[2.0, 1.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    trace_step!(query_vector);
    let key_vector: Vec<f64> =
        normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
            &[1.0, 3.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    trace_step!(key_vector);
    let query_vector: [f64; 2] =
        rotate_vector_coordinate_pair_by_token_position([query_vector[0], query_vector[1]], 2, 0.1);
    trace_step!(query_vector);
    let key_vector: [f64; 2] =
        rotate_vector_coordinate_pair_by_token_position([key_vector[0], key_vector[1]], 1, 0.1);
    trace_step!(key_vector);
    let score: f64 =
        (query_vector[0] * key_vector[0] + query_vector[1] * key_vector[1]) / 2.0_f64.sqrt();
    trace_step!(score);
    assert!(score.is_finite());
    println!("QK-Norm + RoPE score={score:.4}");
}
