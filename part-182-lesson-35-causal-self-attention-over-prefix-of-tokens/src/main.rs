// Урок 35.2. Причинное внимание по предыдущим токенам последовательности.
// Для позиции i softmax вычисляется только по позициям 0..=i.

fn main() {
    lesson_trace::enable();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    lesson_trace::trace_step!(states);
    let context: Vec<[f64; 2]> =
        part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::causal_self_attention_over_query_key_value_sequences(&states, &states, &states)
            .unwrap();
    lesson_trace::trace_step!(context);
    assert_eq!(context[0], states[0]);
    lesson_trace::disable();
    visualize_causal_self_attention_over_prefix_of_tokens(&states);
    for (index, state) in context.iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(state);
        println!("позиция {index}: {state:?}");
    }
}

fn visualize_causal_self_attention_over_prefix_of_tokens(states: &[[f64; 2]]) {
    let matrix: Vec<Vec<f64>> = states
        .iter()
        .enumerate()
        .map(|(item_index, query_vector)| {
            // Оценку модели до преобразования в вероятность называют logit.
            let raw_model_scores: Vec<f64> = (0..=item_index)
                .map(|past_index| {
                    (query_vector[0] * states[past_index][0]
                        + query_vector[1] * states[past_index][1])
                        / 2.0_f64.sqrt()
                })
                .collect();
            let weights: Vec<f64> = part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::softmax_probabilities_from_raw_model_scores(&raw_model_scores);
            (0..states.len())
                .map(|past_index| {
                    if past_index <= item_index {
                        weights[past_index]
                    } else {
                        0.0
                    }
                })
                .collect()
        })
        .collect();
    let path: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "causal-attention",
        "Веса причинного внимания",
        &matrix,
    )
    .expect("график");
    println!("график: {}", path.display());
}
