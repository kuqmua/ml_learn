//! Рекуррентная модель состояния.

/// Линейная рекуррентная модель состояния h_t = a*h_(t-1) + b*x_t.
pub fn scan(input: &[f64], retention_factor: f64, input_factor: f64) -> Vec<f64> {
    let mut state = 0.0;
    input
        .iter()
        .map(|&value| {
            state = retention_factor * state + input_factor * value;
            state
        })
        .collect()
}
