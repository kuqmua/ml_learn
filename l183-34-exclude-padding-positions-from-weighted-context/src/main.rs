// Урок 34.2. Исключение добавленных пустых позиций из взвешенного контекста.
// Связь с принятой терминологией: Исключение PAD токенов из внимания encoder.
// Зачем здесь эта тема: PAD выравнивает длины последовательностей, но не несёт смысла текста.
// Почему код устроен так: Исключаем PAD из оценок внимания до softmax, чтобы он не менял реальные
//   выходы.
// Представь: Добавленные PAD-позиции не должны менять смысл настоящих слов при внимании.
// Добавленные PAD позиции не должны влиять на реальные выходные токены.

fn main() {
    lesson_trace::enable();
    let real: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
    lesson_trace::trace_step!(real);
    let base: Vec<[f64; 2]> =
        l182_34_build_text_context_from_both_earlier_and_later_positions::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
            &real,
            &[true, true],
        )
        .unwrap();
    lesson_trace::trace_step!(base);
    lesson_trace::trace_note!("Добавление пустых позиций к последовательности называют padding.");
    let input_with_padding_rows: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [100.0, 100.0]];
    lesson_trace::trace_step!(input_with_padding_rows);
    lesson_trace::trace_note!("Игнорирование добавленных пустых позиций называют padding mask.");
    let output_ignoring_padding: Vec<[f64; 2]> =
        l182_34_build_text_context_from_both_earlier_and_later_positions::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
            &input_with_padding_rows,
            &[true, true, false],
        )
        .unwrap();
    lesson_trace::trace_step!(output_ignoring_padding);
    assert_eq!(base, output_ignoring_padding[..2]);
    println!("выход без PAD и с PAD совпадает: {base:?}");
}
