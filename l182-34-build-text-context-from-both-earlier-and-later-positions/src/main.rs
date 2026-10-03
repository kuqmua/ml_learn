// Урок 34.1. Построение контекста текста по предыдущим и следующим позициям.
// Связь с принятой терминологией: Двунаправленное внимание между токенами в encoder.
// Зачем здесь эта тема: Для понимания целой фразы токен может использовать слова и слева, и справа.
// Почему код устроен так: Разрешаем двунаправленное внимание, отличая его от маски генеративного
//   decoder.
// Представь: В фразе «он сел» слово «он» может учитывать и слова после него, если задача — понять
//   всю фразу.
// В отличие от decoder, текущий токен читает контекст и слева, и справа.

use l182_34_build_text_context_from_both_earlier_and_later_positions::calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores;

fn main() {
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let output: [[f64; 2]; 3] =
        calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores(
            &states, &[true; 3],
        )
        .unwrap();
    let changed: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [9.0, 9.0]];

    assert_ne!(
        output[0],
        calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores(
            &changed, &[true; 3],
        )
        .unwrap()[0]
    );
    let _ = &(output[0]);
}
