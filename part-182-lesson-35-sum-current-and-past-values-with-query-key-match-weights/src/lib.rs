//! Урок 182. Сложение текущих и прошлых значений с весами по совпадению запросов и ключей.
//! Связь с принятой терминологией: Причинное self-attention.

/// Устойчивый softmax для конечных логитов.
// Оценку модели до преобразования в вероятность называют logit.
/// Softmax: вычитаем максимальную оценку, вычисляем экспоненты и делим каждую на их сумму.
pub fn calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
    raw_model_scores: &[f64],
) -> Vec<f64> {
    let maximum: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    lesson_trace::trace_step!(maximum);
    let weights: Vec<f64> = raw_model_scores
        .iter()
        .map(|&value| (value - maximum).exp())
        .collect();
    lesson_trace::trace_step!(weights);
    let total: f64 = weights.iter().sum();
    lesson_trace::trace_step!(total);
    weights.into_iter().map(|weight| weight / total).collect()
}

/// Один причинный head. Строка i видит только j <= i.
/// Причинное внимание: совпадения запроса и ключей делим на sqrt(2), превращаем в веса через softmax и суммируем значения только текущей и прошлых позиций.
pub fn calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
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
    let mut output: Vec<[f64; 2]> = Vec::with_capacity(query_vector.len());
    lesson_trace::trace_step!(output);
    for index in 0..query_vector.len() {
        lesson_trace::trace_step!(index);
        // Ключи после index скрыты причинной маской; 2 под корнем — размерность Q и K.
        // Деление на √2 удерживает величину dot product при переходе к softmax.
        let raw_model_scores: Vec<f64> = (0..=index)
            .map(|past| {
                (query_vector[index][0] * key_vector[past][0]
                    + query_vector[index][1] * key_vector[past][1])
                    / 2.0_f64.sqrt()
            })
            .collect();
        lesson_trace::trace_step!(raw_model_scores);
        let weights: Vec<f64> =
            calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(&raw_model_scores);
        lesson_trace::trace_step!(weights);
        let mut state: [f64; 2] = [0.0; 2];
        lesson_trace::trace_step!(state);
        for (past, &weight) in weights.iter().enumerate() {
            lesson_trace::trace_step!(past);
            lesson_trace::trace_step!(weight);
            for feature in 0..2 {
                lesson_trace::trace_step!(feature);
                state[feature] += weight * value_vectors[past][feature];
                lesson_trace::trace_step!(state);
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
        let query_vector: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
        let key_vector: [[f64; 2]; 2] = query_vector;
        let first: Vec<[f64; 2]> =
            super::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
                &query_vector,
                &key_vector,
                &[[2.0, 3.0], [4.0, 5.0]],
            )
            .unwrap();
        let second: Vec<[f64; 2]> =
            super::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
                &query_vector,
                &key_vector,
                &[[2.0, 3.0], [999.0, 999.0]],
            )
            .unwrap();
        assert_eq!(first[0], second[0]);
        assert_eq!(first[0], [2.0, 3.0]);
    }
}
