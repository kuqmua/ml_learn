// Урок 34.2. Исключение добавленных пустых позиций из взвешенного контекста.
// Зачем здесь эта тема: PAD выравнивает длины последовательностей, но не несёт смысла текста.
// Почему код устроен так: Исключаем PAD из оценок внимания до softmax, чтобы он не менял реальные
//   выходы.
// Представь: Добавленные PAD-позиции не должны менять смысл настоящих слов при внимании.
// Добавленные PAD позиции не должны влиять на реальные выходные токены.

use l182_34_calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores::calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores;

fn main() {
    let real: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];

    let input_with_padding_rows: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [100.0, 100.0]];

    assert_eq!(
        calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores(
            &real,
            &[true, true],
        )
        .unwrap(),
        calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores(
            &input_with_padding_rows,
            &[true, true, false],
        )
        .unwrap()[..2]
    );
}
