//! Рекуррентная модель состояния.

/// Линейная рекуррентная модель состояния h_t = a*h_(t-1) + b*x_t.
// Долю (fraction) предыдущего состояния, сохраняемую на следующем шаге, называют retention.
pub fn calculate_linear_recurrent_state_sequence_from_inputs(
    input: &[f64],
    previous_state_share_kept: f64,
    input_factor: f64,
) -> Vec<f64> {
    let mut state = 0.0;
    input
        .iter()
        .map(|&value| {
            state = previous_state_share_kept * state + input_factor * value;
            state
        })
        .collect()
}
