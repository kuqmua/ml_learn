//! Скрытое состояние RNN.

/// Скалярная RNN: h_t = tanh(w_x*x_t + w_h*h_(t-1)).
pub fn states(input: &[f64], wx: f64, wh: f64) -> Vec<f64> {
    let mut h = 0.0;
    input
        .iter()
        .map(|&x| {
            h = (wx * x + wh * h).tanh();
            h
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::states;
    #[test]
    fn future_does_not_change_past() {
        let short = states(&[1.0, 2.0], 0.4, 0.6);
        let long = states(&[1.0, 2.0, 999.0], 0.4, 0.6);
        assert_eq!(short, long[..2]);
    }
}
