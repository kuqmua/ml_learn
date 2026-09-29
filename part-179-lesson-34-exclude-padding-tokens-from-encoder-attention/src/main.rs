// Урок 34.2. Исключение PAD токенов из внимания encoder.
// Добавленные PAD позиции не должны влиять на реальные выходные токены.

fn main() {
    let real = [[1.0, 0.0], [0.0, 1.0]];
    let base =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &real,
            &[true, true],
        )
        .unwrap();
    // Добавление пустых позиций к последовательности называют padding.
    let input_with_padding_rows = [[1.0, 0.0], [0.0, 1.0], [100.0, 100.0]];
    // Игнорирование добавленных пустых позиций называют padding mask.
    let output_ignoring_padding =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &input_with_padding_rows,
            &[true, true, false],
        )
        .unwrap();
    assert_eq!(base, output_ignoring_padding[..2]);
    println!("выход без PAD и с PAD совпадает: {base:?}");
}
