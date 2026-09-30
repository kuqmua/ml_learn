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

fn main() {
    let query_vector: [f64; 2] =
        normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
            &[2.0, 1.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let key_vector: [f64; 2] =
        normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
            &[1.0, 3.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let query_vector: [f64; 2] =
        rotate_vector_coordinate_pair_by_token_position([query_vector[0], query_vector[1]], 2, 0.1);
    let key_vector: [f64; 2] =
        rotate_vector_coordinate_pair_by_token_position([key_vector[0], key_vector[1]], 1, 0.1);
    let score: f64 =
        (query_vector[0] * key_vector[0] + query_vector[1] * key_vector[1]) / 2.0_f64.sqrt();
    assert!(score.is_finite());
}
