// Урок 36.6. Кэширование векторов ключей и значений прошлых токенов при генерации.
// Сохраняем K/V прошлых токенов и сверяем последний выход с полным причинным пересчётом.

fn main() {
    let states = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let full =
        part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::causal_self_attention_over_query_key_value_sequences(&states, &states, &states)
            .unwrap();
    let mut cached_keys = Vec::new();
    let mut cached_values = Vec::new();
    let mut cached_outputs = Vec::new();
    for &new_state in &states {
        cached_keys.push(new_state);
        cached_values.push(new_state);
        // Оценку модели до преобразования в вероятность называют logit.
        let raw_model_scores: Vec<f64> = cached_keys
            .iter()
            .map(|key| (new_state[0] * key[0] + new_state[1] * key[1]) / 2.0_f64.sqrt())
            .collect();
        let weights =
            part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::softmax_probabilities_from_raw_model_scores(
                &raw_model_scores,
            );
        let output = weights.iter().zip(&cached_values).fold(
            [0.0; 2],
            |mut output_state, (&weight_value, cached_value)| {
                output_state[0] += weight_value * cached_value[0];
                output_state[1] += weight_value * cached_value[1];
                output_state
            },
        );
        cached_outputs.push(output);
    }
    for (cached, recomputed) in cached_outputs.iter().zip(full) {
        assert!((cached[0] - recomputed[0]).abs() < 1e-12);
        assert!((cached[1] - recomputed[1]).abs() < 1e-12);
    }
    println!("выходы с KV-cache: {cached_outputs:?}");
}
