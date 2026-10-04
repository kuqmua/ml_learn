// Урок 195. Поворачиваем пары чисел, чтобы сравнение запроса и ключа учитывало их места в тексте.
// Угол поворота равен номеру места, умноженному на заданный шаг угла в радианах.
// cos задаёт вклад прежней координаты, sin — вклад другой координаты.
// Поворот сохраняет длину стрелки. Если обе стрелки повернуть на одинаковый угол,
// сумма произведений соответствующих координат не изменится.
// Если углы разные, результат сравнения может измениться.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

/// Умножаем первое число на первое, второе на второе и так далее, затем складываем результаты.
use l195_36_rotate_vec_coord_pair_by_token_position::rotate_vec_coord_pair_by_token_position;

fn multiply_matching_coords_then_add_results(value1: [f64; 2], value2: [f64; 2]) -> f64 {
    value1[0] * value2[0] + value1[1] * value2[1]
}
fn main() {
    let query_vec: [f64; 2] = [1.0, 0.0];
    let key_vec: [f64; 2] = [1.0, 0.0];
    let query_key_match_after_equal_position_rotation: f64 =
        multiply_matching_coords_then_add_results(
            rotate_vec_coord_pair_by_token_position(query_vec, 3, 0.2),
            rotate_vec_coord_pair_by_token_position(key_vec, 3, 0.2),
        );

    assert!(check_f64_eq_1e_minus_12(
        query_key_match_after_equal_position_rotation,
        1.0
    ));
    assert!(
        multiply_matching_coords_then_add_results(
            rotate_vec_coord_pair_by_token_position(query_vec, 3, 0.2),
            rotate_vec_coord_pair_by_token_position(key_vec, 8, 0.2),
        ) < query_key_match_after_equal_position_rotation
    );
}

// Чему учит этот урок:
// Учимся учитывать позицию поворотом пары координат запроса и ключа.
// Проверяем сохранение их совпадения при одинаковом повороте и изменение оценки при разных
// позициях.
