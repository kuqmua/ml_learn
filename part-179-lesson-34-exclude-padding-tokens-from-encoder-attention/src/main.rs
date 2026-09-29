// Урок 34.2. Исключение PAD токенов из внимания encoder.
// Добавленные PAD позиции не должны влиять на реальные выходные токены.

fn main() {
    lesson_trace::enable();
    let real: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
    lesson_trace::trace_step!(real);
    let base: Vec<[f64; 2]> =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &real,
            &[true, true],
        )
        .unwrap();
    lesson_trace::trace_step!(base);
    // Добавление пустых позиций к последовательности называют padding.
    let input_with_padding_rows: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [100.0, 100.0]];
    lesson_trace::trace_step!(input_with_padding_rows);
    // Игнорирование добавленных пустых позиций называют padding mask.
    let output_ignoring_padding: Vec<[f64; 2]> =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &input_with_padding_rows,
            &[true, true, false],
        )
        .unwrap();
    lesson_trace::trace_step!(output_ignoring_padding);
    assert_eq!(base, output_ignoring_padding[..2]);
    println!("выход без PAD и с PAD совпадает: {base:?}");
}
