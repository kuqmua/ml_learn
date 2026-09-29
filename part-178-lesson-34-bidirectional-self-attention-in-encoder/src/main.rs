// Урок 34.1. Двунаправленное внимание между токенами в encoder.
// Почему этот урок сейчас: Для понимания целой фразы токен может использовать слова и слева, и справа.
// Почему пример устроен так: Разрешаем двунаправленное внимание, отличая его от маски генеративного decoder.
// В отличие от decoder, текущий токен читает контекст и слева, и справа.

fn main() {
    lesson_trace::enable();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    lesson_trace::trace_step!(states);
    let output: Vec<[f64; 2]> =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &states, &[true; 3],
        )
        .unwrap();
    lesson_trace::trace_step!(output);
    let changed: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [9.0, 9.0]];
    lesson_trace::trace_step!(changed);
    let after: Vec<[f64; 2]> =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &changed, &[true; 3],
        )
        .unwrap();
    lesson_trace::trace_step!(after);
    assert_ne!(output[0], after[0]);
    println!("первый токен учитывает будущий контекст: {:?}", output[0]);
}
