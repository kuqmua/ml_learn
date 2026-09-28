//! Учебный прямой проход GPT с фиксированными весами.

use part_177_lesson_34_causal_self_attention::causal_attention;

/// Возвращает состояния после причинного внимания и residual.
pub fn hidden_states(ids: &[usize]) -> Vec<[f64; 2]> {
    let embedding = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    let states: Vec<[f64; 2]> = ids
        .iter()
        .enumerate()
        .map(|(position, &id)| [embedding[id][0] + position as f64 * 0.1, embedding[id][1]])
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
pub fn forward(ids: &[usize]) -> Vec<[f64; 3]> {
    hidden_states(ids)
        .into_iter()
        .map(|hidden| [hidden[0], hidden[1], (hidden[0] + hidden[1]) * 0.5])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{forward, hidden_states};

    #[test]
    fn prefix_is_causal() {
        assert_eq!(hidden_states(&[0])[0], hidden_states(&[0, 1])[0]);
        assert_eq!(forward(&[0])[0], forward(&[0, 1])[0]);
    }
}
