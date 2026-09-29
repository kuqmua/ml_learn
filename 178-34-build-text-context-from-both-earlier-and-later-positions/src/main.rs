// Урок 34.1. Построение контекста текста по предыдущим и следующим позициям.
// Связь с принятой терминологией: Двунаправленное внимание между токенами в encoder.
// Зачем здесь эта тема: Для понимания целой фразы токен может использовать слова и слева, и справа.
// Почему код устроен так: Разрешаем двунаправленное внимание, отличая его от маски генеративного
//   decoder.
// Представь: В фразе «он сел» слово «он» может учитывать и слова после него, если задача — понять
//   всю фразу.
// В отличие от decoder, текущий токен читает контекст и слева, и справа.

fn main() {
    lesson_trace::enable();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    lesson_trace::trace_step!(states);
    let output: Vec<[f64; 2]> =
        l178_34_build_text_context_from_both_earlier_and_later_positions::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
            &states, &[true; 3],
        )
        .unwrap();
    lesson_trace::trace_step!(output);
    let changed: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [9.0, 9.0]];
    lesson_trace::trace_step!(changed);
    let after: Vec<[f64; 2]> =
        l178_34_build_text_context_from_both_earlier_and_later_positions::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
            &changed, &[true; 3],
        )
        .unwrap();
    lesson_trace::trace_step!(after);
    assert_ne!(output[0], after[0]);
    println!("первый токен учитывает будущий контекст: {:?}", output[0]);
}
