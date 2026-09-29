// Урок 36.3. Совместное использование голов ключей и значений несколькими головами запросов.
// Несколько Q-голов совместно используют меньшее число K/V-голов.

fn main() {
    // Четырём Q-головам соответствуют две K/V-головы.
    let queries: [[f64; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [-1.0, 1.0]];
    let keys: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.0, 1.0], [1.0, 0.0]]];
    let values: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.2, 0.8], [0.8, 0.2]]];
    let mut output: Vec<[f64; 2]> = Vec::new();
    for (head, &query) in queries.iter().enumerate() {
        let group: usize = head / 2;
        // Оценку модели до преобразования в вероятность называют logit.
        let raw_model_scores: Vec<f64> = keys[group]
            .iter()
            .map(|key_vector| query[0] * key_vector[0] + query[1] * key_vector[1])
            .collect();
        let weights: Vec<f64> =
            part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::softmax_probabilities_from_raw_model_scores(
                &raw_model_scores,
            );
        output.push([
            weights[0] * values[group][0][0] + weights[1] * values[group][1][0],
            weights[0] * values[group][0][1] + weights[1] * values[group][1][1],
        ]);
    }
    assert_eq!(output.len(), 4);
    println!("4 Q / 2 KV головы: {output:?}");
}
