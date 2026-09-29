// Урок 35.3. Объединение нескольких голов внимания.
// Разные головы получают собственные проекции и соединяются перед выходной проекцией.

fn main() {
    let input: [[f64; 2]; 3] = [[1.0, 2.0], [3.0, 1.0], [2.0, 4.0]];
    // Первая голова смотрит на первый признак, вторая — на второй.
    let first_attention_head: Vec<[f64; 2]> = input
        .iter()
        .map(|input_value| [input_value[0], 0.0])
        .collect();
    let second_attention_head: Vec<[f64; 2]> = input
        .iter()
        .map(|input_value| [0.0, input_value[1]])
        .collect();
    let first_output: Vec<[f64; 2]> = part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::causal_self_attention_over_query_key_value_sequences(
        &first_attention_head,
        &first_attention_head,
        &first_attention_head,
    )
    .unwrap();
    let second_output: Vec<[f64; 2]> = part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::causal_self_attention_over_query_key_value_sequences(
        &second_attention_head,
        &second_attention_head,
        &second_attention_head,
    )
    .unwrap();
    // Конкатенация двух одномерных выходов здесь сразу даёт размерность 2.
    let combined: Vec<[f64; 2]> = first_output
        .iter()
        .zip(&second_output)
        .map(|(first_value, second_value)| [first_value[0], second_value[1]])
        .collect();
    assert_eq!(combined[0], input[0]);
    println!("две головы: {combined:?}");
}
