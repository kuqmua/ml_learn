//! Учебный прямой проход GPT с фиксированными весами.

use part_182_lesson_35_causal_self_attention::causal_attention;

/// Возвращает состояния после причинного внимания и residual.
// Единицу текста, которую модель обрабатывает как одно целое, называют token.
pub fn calculate_hidden_states_for_text_units(text_unit_identifiers: &[usize]) -> Vec<[f64; 2]> {
    // Плотное числовое представление объекта называют embedding.
    let dense_numeric_representation = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
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
    let context = causal_attention(&states, &states, &states).unwrap();
    states
        .iter()
        .zip(&context)
        .map(|(&state, &attended)| [state[0] + attended[0], state[1] + attended[1]])
        .collect()
}

/// Применяет фиксированную выходную проекцию к состояниям decoder.
pub fn calculate_forward_pass_for_text_units(text_unit_identifiers: &[usize]) -> Vec<[f64; 3]> {
    calculate_hidden_states_for_text_units(text_unit_identifiers)
        .into_iter()
        .map(|hidden| [hidden[0], hidden[1], (hidden[0] + hidden[1]) * 0.5])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{calculate_forward_pass_for_text_units, calculate_hidden_states_for_text_units};

    #[test]
    fn prefix_is_causal() {
        assert_eq!(
            calculate_hidden_states_for_text_units(&[0])[0],
            calculate_hidden_states_for_text_units(&[0, 1])[0]
        );
        assert_eq!(
            calculate_forward_pass_for_text_units(&[0])[0],
            calculate_forward_pass_for_text_units(&[0, 1])[0]
        );
    }
}
