//! Урок 187. Контекст без будущих данных: сложение текущих и прошлых значений с весами совпадений запроса и ключей.
//! Связь с принятой терминологией: Причинное self-attention.

/// Один причинный head. Строка i видит только j <= i.
/// Причинное внимание: совпадения запроса и ключей делим на sqrt(2), превращаем в веса через softmax и суммируем значения только текущей и прошлых позиций.
/// Длина текста задаётся во время выполнения; совпадение длин Q/K/V проверяется здесь.
use l186_35_calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares;

pub fn calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
    query_vec: &[[f64; 2]],
    key_vec: &[[f64; 2]],
    value_vecs: &[[f64; 2]],
) -> Result<Vec<[f64; 2]>, &'static str> {
    if query_vec.len() != key_vec.len() || key_vec.len() != value_vecs.len() || query_vec.is_empty()
    {
        return Err("Q, K и V должны быть непустыми последовательностями одинаковой длины");
    }
    let mut output: Vec<[f64; 2]> = Vec::with_capacity(query_vec.len());
    for index in 0..query_vec.len() {
        let mut state: [f64; 2] = [0.0; 2];
        for (past, weight) in calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
            &(0..=index)
                .map(|past| {
                    (query_vec[index][0] * key_vec[past][0]
                        + query_vec[index][1] * key_vec[past][1])
                        / 2.0_f64.sqrt()
                })
                .collect::<Vec<_>>(),
        )
        .into_iter()
        .enumerate()
        {
            for feature in 0..2 {
                state[feature] += weight * value_vecs[past][feature];
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
        let query_vec: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
        let key_vec: [[f64; 2]; 2] = query_vec;
        let context_with_original_future: Vec<[f64; 2]> =
            super::calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
                &query_vec,
                &key_vec,
                &[[2.0, 3.0], [4.0, 5.0]],
            )
            .unwrap();

        assert_eq!(
            context_with_original_future[0],
            super::calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
                &query_vec,
                &key_vec,
                &[[2.0, 3.0], [999.0, 999.0]],
            )
            .unwrap()[0]
        );
        assert_eq!(context_with_original_future[0], [2.0, 3.0]);
    }
}
