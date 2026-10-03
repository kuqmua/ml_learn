// Урок 195. Поворачиваем пары чисел, чтобы сравнение запроса и ключа учитывало их места в тексте.
// Угол поворота равен номеру места, умноженному на заданный шаг угла в радианах.
// cos задаёт вклад прежней координаты, sin — вклад другой координаты.
// Поворот сохраняет длину стрелки. Если обе стрелки повернуть на одинаковый угол,
// сумма произведений соответствующих координат не изменится.
// Если углы разные, результат сравнения может измениться.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

/// Умножаем первое число на первое, второе на второе и так далее, затем складываем результаты.
use l195_36_rotate_vec_coord_pair_by_token_position::rotate_vec_coord_pair_by_token_position;

fn multiply_matching_coords_then_add_results(first_value: [f64; 2], second_value: [f64; 2]) -> f64 {
    first_value[0] * second_value[0] + first_value[1] * second_value[1]
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

    plot_sum_after_multiplying_rotated_coords_for_relative_positions();
}

fn plot_sum_after_multiplying_rotated_coords_for_relative_positions() {
    let query_vec: [f64; 2] = [1.0, 0.0];
    let query_vec: [f64; 2] = rotate_vec_coord_pair_by_token_position(query_vec, 0, 0.2);

    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "rope-relative",
        "RoPE и расстояние между позициями",
        "сдвиг позиции",
        "сумма произведений координат",
        &[lesson_visualization::Series {
            name: "score",
            points: &(0..=20)
                .map(|position_index| {
                    (
                        position_index as f64,
                        multiply_matching_coords_then_add_results(
                            query_vec,
                            rotate_vec_coord_pair_by_token_position(
                                [1.0, 0.0],
                                position_index,
                                0.2,
                            ),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
