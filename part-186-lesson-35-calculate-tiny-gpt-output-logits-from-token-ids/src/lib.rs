//! Учебный прямой проход GPT с фиксированными весами.

/// Возвращает состояния после причинного внимания и residual.
// Единицу текста, которую модель обрабатывает как одно целое, называют token.
pub fn calculate_decoder_hidden_states_for_token_ids(
    text_unit_identifiers: &[usize],
) -> Vec<[f64; 2]> {
    // Плотное числовое представление объекта называют embedding.
    let dense_numeric_representation: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    let states: Vec<[f64; 2]> = text_unit_identifiers
        .iter()
        .enumerate()
        .map(|(position, &item_identifier)| {
            [
                dense_numeric_representation[item_identifier][0] + position as f64 * 0.1,
                dense_numeric_representation[item_identifier][1],
            ]
        })
        .collect();
    if states.is_empty() {
        return Vec::new();
    }
    let context: Vec<[f64; 2]> =
        part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::causal_self_attention_over_query_key_value_sequences(&states, &states, &states)
            .unwrap();
    states
        .iter()
        .zip(&context)
        .map(|(&state, &attended)| [state[0] + attended[0], state[1] + attended[1]])
        .collect()
}

/// Применяет фиксированную выходную проекцию к состояниям decoder.
pub fn calculate_decoder_output_logits_for_token_ids(
    text_unit_identifiers: &[usize],
) -> Vec<[f64; 3]> {
    calculate_decoder_hidden_states_for_token_ids(text_unit_identifiers)
        .into_iter()
        .map(|hidden| [hidden[0], hidden[1], (hidden[0] + hidden[1]) * 0.5])
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn prefix_is_causal() {
        assert_eq!(
            super::calculate_decoder_hidden_states_for_token_ids(&[0])[0],
            super::calculate_decoder_hidden_states_for_token_ids(&[0, 1])[0]
        );
        assert_eq!(
            super::calculate_decoder_output_logits_for_token_ids(&[0])[0],
            super::calculate_decoder_output_logits_for_token_ids(&[0, 1])[0]
        );
    }
}
