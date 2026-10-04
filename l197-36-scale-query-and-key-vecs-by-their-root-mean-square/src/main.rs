// Урок 197. Соединять нормализацию запроса и ключа, позиционный поворот и оценку совпадения.
// Проверяем, что увеличение масштаба исходного запроса почти не меняет нормализованное
// представление.

use l194_36_normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights::normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights;
use l195_36_rotate_vec_coord_pair_by_token_position::rotate_vec_coord_pair_by_token_position;

fn main() {
    let query_vec: [f64; 2] =
        normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
            &[2.0, 1.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let key_vec: [f64; 2] =
        normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
            &[1.0, 3.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let query_vec: [f64; 2] =
        rotate_vec_coord_pair_by_token_position([query_vec[0], query_vec[1]], 2, 0.1);
    let key_vec: [f64; 2] =
        rotate_vec_coord_pair_by_token_position([key_vec[0], key_vec[1]], 1, 0.1);
    let scaled_query_key_match: f64 =
        (query_vec[0] * key_vec[0] + query_vec[1] * key_vec[1]) / 2.0_f64.sqrt();
    assert!(scaled_query_key_match.is_finite());

    println!(
        "Нормализованный и повёрнутый запрос={query_vec:?}; ключ={key_vec:?}; совпадение={scaled_query_key_match}"
    );
    let scaled_input =
        normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
            &[20.0, 10.0],
            &[1.0, 1.0],
            1e-6,
        )
        .unwrap();
    let rotated = rotate_vec_coord_pair_by_token_position(scaled_input, 2, 0.1);
    for i in 0..2 {
        assert!((rotated[i] - query_vec[i]).abs() < 1e-6);
    }
    println!(
        "Увеличение масштаба входного запроса в 10 раз почти не изменило результат нормализации."
    );
}

// Чему учит этот урок:
// Учимся соединять нормализацию запроса и ключа, позиционный поворот и оценку совпадения.
// Проверяем, что увеличение масштаба исходного запроса почти не меняет нормализованное
// представление.
