//! Рекуррентная модель состояния.

/// Линейная рекуррентная модель состояния h_t = a*h_(t-1) + b*x_t.
pub fn scan(input: &[f64], a: f64, b: f64) -> Vec<f64> {
    let mut state = 0.0;
    input
        .iter()
        .map(|&value| {
            state = a * state + b * value;
            state
        })
        .collect()
}
