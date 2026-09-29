//! Причинное self-attention.

/// Устойчивый softmax для конечных логитов.
// Оценку модели до преобразования в вероятность называют logit.
pub fn softmax_probabilities_from_raw_model_scores(raw_model_scores: &[f64]) -> Vec<f64> {
    let maximum = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let weights: Vec<f64> = raw_model_scores
        .iter()
        .map(|&value| (value - maximum).exp())
        .collect();
    let total: f64 = weights.iter().sum();
    weights.into_iter().map(|weight| weight / total).collect()
}

/// Один причинный head. Строка i видит только j <= i.
pub fn causal_self_attention_over_query_key_value_sequences(
    query_vector: &[[f64; 2]],
    key_vector: &[[f64; 2]],
    value_vectors: &[[f64; 2]],
) -> Result<Vec<[f64; 2]>, &'static str> {
    if query_vector.len() != key_vector.len()
        || key_vector.len() != value_vectors.len()
        || query_vector.is_empty()
    {
        return Err("неверная форма Q/K/V");
    }
    let mut output = Vec::with_capacity(query_vector.len());
    for index in 0..query_vector.len() {
        let raw_model_scores: Vec<f64> = (0..=index)
            .map(|past| {
                (query_vector[index][0] * key_vector[past][0]
                    + query_vector[index][1] * key_vector[past][1])
                    / 2.0_f64.sqrt()
            })
            .collect();
        let weights = softmax_probabilities_from_raw_model_scores(&raw_model_scores);
        let mut state = [0.0; 2];
        for (past, &weight) in weights.iter().enumerate() {
            for feature in 0..2 {
                state[feature] += weight * value_vectors[past][feature];
            }
        }
        output.push(state);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_output_ignores_future_values() {
        let query_vector = [[1.0, 0.0], [0.0, 1.0]];
        let key_vector = query_vector;
        let first = super::causal_self_attention_over_query_key_value_sequences(
            &query_vector,
            &key_vector,
            &[[2.0, 3.0], [4.0, 5.0]],
        )
        .unwrap();
        let second = super::causal_self_attention_over_query_key_value_sequences(
            &query_vector,
            &key_vector,
            &[[2.0, 3.0], [999.0, 999.0]],
        )
        .unwrap();
        assert_eq!(first[0], second[0]);
        assert_eq!(first[0], [2.0, 3.0]);
    }
}
