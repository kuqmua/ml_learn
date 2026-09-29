// Урок 36.6. Кэширование векторов ключей и значений прошлых токенов при генерации.
// Сохраняем K/V прошлых токенов и сверяем последний выход с полным причинным пересчётом.

fn main() {
    lesson_trace::enable();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    lesson_trace::trace_step!(states);
    let full: Vec<[f64; 2]> =
        part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::causal_self_attention_over_query_key_value_sequences(&states, &states, &states)
            .unwrap();
    lesson_trace::trace_step!(full);
    let mut cached_keys: Vec<[f64; 2]> = Vec::new();
    lesson_trace::trace_step!(cached_keys);
    let mut cached_values: Vec<[f64; 2]> = Vec::new();
    lesson_trace::trace_step!(cached_values);
    let mut cached_outputs: Vec<[f64; 2]> = Vec::new();
    lesson_trace::trace_step!(cached_outputs);
    for &new_state in &states {
        lesson_trace::trace_step!(new_state);
        cached_keys.push(new_state);
        cached_values.push(new_state);
        // Оценку модели до преобразования в вероятность называют logit.
        let raw_model_scores: Vec<f64> = cached_keys
            .iter()
            .map(|key| (new_state[0] * key[0] + new_state[1] * key[1]) / 2.0_f64.sqrt())
            .collect();
        lesson_trace::trace_step!(raw_model_scores);
        let weights: Vec<f64> =
            part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::softmax_probabilities_from_raw_model_scores(
                &raw_model_scores,
            );
        lesson_trace::trace_step!(weights);
        let output: [f64; 2] = weights.iter().zip(&cached_values).fold(
            [0.0; 2],
            |mut output_state, (&weight_value, cached_value)| {
                output_state[0] += weight_value * cached_value[0];
                lesson_trace::trace_step!(output_state);
                output_state[1] += weight_value * cached_value[1];
                lesson_trace::trace_step!(output_state);
                output_state
            },
        );
        lesson_trace::trace_step!(output);
        cached_outputs.push(output);
    }
    for (cached, recomputed) in cached_outputs.iter().zip(full) {
        lesson_trace::trace_step!(cached);
        lesson_trace::trace_step!(recomputed);
        assert!((cached[0] - recomputed[0]).abs() < 1e-12);
        assert!((cached[1] - recomputed[1]).abs() < 1e-12);
    }
    println!("выходы с KV-cache: {cached_outputs:?}");
}
