//! Урок 187. Контекст без будущих данных: сложение текущих и прошлых значений с весами совпадений запроса и ключей.
//! Связь с принятой терминологией: Причинное self-attention.

/// Один причинный head. Строка i видит только j <= i.
/// Причинное внимание: совпадения запроса и ключей делим на sqrt(2), превращаем в веса через softmax и суммируем значения только текущей и прошлых позиций.
/// Длина текста задаётся во время выполнения; совпадение длин Q/K/V проверяется здесь.
use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum;

pub fn calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
    query_vector: &[[f64; 2]],
    key_vector: &[[f64; 2]],
    value_vectors: &[[f64; 2]],
) -> Result<Vec<[f64; 2]>, &'static str> {
    if query_vector.len() != key_vector.len()
        || key_vector.len() != value_vectors.len()
        || query_vector.is_empty()
    {
        return Err("Q, K и V должны быть непустыми последовательностями одинаковой длины");
    }
    let mut output: Vec<[f64; 2]> = Vec::with_capacity(query_vector.len());
    for index in 0..query_vector.len() {
        let mut state: [f64; 2] = [0.0; 2];
        for (past, weight) in calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &(0..=index)
                .map(|past| {
                    (query_vector[index][0] * key_vector[past][0]
                        + query_vector[index][1] * key_vector[past][1])
                        / 2.0_f64.sqrt()
                })
                .collect::<Vec<_>>(),
        )
        .into_iter()
        .enumerate()
        {
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
        let query_vector: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
        let key_vector: [[f64; 2]; 2] = query_vector;
        let context_with_original_future: Vec<[f64; 2]> =
            super::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
                &query_vector,
                &key_vector,
                &[[2.0, 3.0], [4.0, 5.0]],
            )
            .unwrap();

        assert_eq!(
            context_with_original_future[0],
            super::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
                &query_vector,
                &key_vector,
                &[[2.0, 3.0], [999.0, 999.0]],
            )
            .unwrap()[0]
        );
        assert_eq!(context_with_original_future[0], [2.0, 3.0]);
    }
}
