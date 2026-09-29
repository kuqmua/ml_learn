//! Скрытое состояние RNN.

/// Скалярная RNN: h_t = tanh(w_x*x_t + w_h*h_(t-1)).
pub fn calculate_recurrent_hidden_states_from_input_sequence(
    input: &[f64],
    input_weight: f64,
    recurrent_weight: f64,
) -> Vec<f64> {
    let mut hidden_state: f64 = 0.0;
    input
        .iter()
        .map(|&input_value| {
            hidden_state = (input_weight * input_value + recurrent_weight * hidden_state).tanh();
            hidden_state
        })
        .collect()
}
#[cfg(test)]
mod tests {
    #[test]
    fn future_does_not_change_past() {
        let short: Vec<f64> =
            super::calculate_recurrent_hidden_states_from_input_sequence(&[1.0, 2.0], 0.4, 0.6);
        let long: Vec<f64> = super::calculate_recurrent_hidden_states_from_input_sequence(
            &[1.0, 2.0, 999.0],
            0.4,
            0.6,
        );
        assert_eq!(short, long[..2]);
    }
}
