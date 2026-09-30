// Урок 35.4. Объединение контекстов от нескольких способов сравнения запросов и ключей.
// Связь с принятой терминологией: Объединение нескольких голов внимания.
// Зачем здесь эта тема: Одна голова внимания даёт один набор связей; несколько голов могут выделять
//   разные отношения.
// Почему код устроен так: Считаем головы отдельно и объединяем их выходы в исходную размерность.
// Представь: Одна голова может выделить соседнее слово, другая — более далёкую связь; затем их
//   результаты объединяются.
// Разные головы получают собственные проекции и соединяются перед выходной проекцией.

fn main() {
    lesson_trace::enable();
    let input: [[f64; 2]; 3] = [[1.0, 2.0], [3.0, 1.0], [2.0, 4.0]];
    lesson_trace::trace_step!(input);
    lesson_trace::trace_note!("Первая голова смотрит на первый признак, вторая — на второй.");
    let first_attention_head: Vec<[f64; 2]> = input
        .iter()
        .map(|input_value| [input_value[0], 0.0])
        .collect();
    lesson_trace::trace_step!(first_attention_head);
    let second_attention_head: Vec<[f64; 2]> = input
        .iter()
        .map(|input_value| [0.0, input_value[1]])
        .collect();
    lesson_trace::trace_step!(second_attention_head);
    let first_output: Vec<[f64; 2]> = l187_35_calculate_past_context_by_summing_current_and_past_values_with_match_weights::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
        &first_attention_head,
        &first_attention_head,
        &first_attention_head,
    )
    .unwrap();
    lesson_trace::trace_step!(first_output);
    let second_output: Vec<[f64; 2]> = l187_35_calculate_past_context_by_summing_current_and_past_values_with_match_weights::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
        &second_attention_head,
        &second_attention_head,
        &second_attention_head,
    )
    .unwrap();
    lesson_trace::trace_step!(second_output);
    lesson_trace::trace_note!(
        "Конкатенация двух одномерных выходов здесь сразу даёт размерность 2."
    );
    let combined: Vec<[f64; 2]> = first_output
        .iter()
        .zip(&second_output)
        .map(|(first_value, second_value)| [first_value[0], second_value[1]])
        .collect();
    lesson_trace::trace_step!(combined);
    assert_eq!(combined[0], input[0]);
    println!("две головы: {combined:?}");
}
