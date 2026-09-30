// Урок 34.1. Построение контекста текста по предыдущим и следующим позициям.
// Связь с принятой терминологией: Двунаправленное внимание между токенами в encoder.
// Зачем здесь эта тема: Для понимания целой фразы токен может использовать слова и слева, и справа.
// Почему код устроен так: Разрешаем двунаправленное внимание, отличая его от маски генеративного
//   decoder.
// Представь: В фразе «он сел» слово «он» может учитывать и слова после него, если задача — понять
//   всю фразу.
// В отличие от decoder, текущий токен читает контекст и слева, и справа.

use l182_34_build_text_context_from_both_earlier_and_later_positions::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products;

use lesson_trace::{enable, trace_step};

fn main() {
    enable();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    trace_step!(states);
    let output: Vec<[f64; 2]> =
        calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
            &states, &[true; 3],
        )
        .unwrap();
    trace_step!(output);
    let changed: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [9.0, 9.0]];
    trace_step!(changed);
    let after: Vec<[f64; 2]> =
        calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
            &changed, &[true; 3],
        )
        .unwrap();
    trace_step!(after);
    assert_ne!(output[0], after[0]);
    println!("первый токен учитывает будущий контекст: {:?}", output[0]);
}
