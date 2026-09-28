//! Прямой процесс диффузии.

/// Прямой шаг диффузии при заранее выбранном шуме epsilon.
pub fn add_noise(
    clean: f64,
    epsilon: f64,
    cumulative_signal_retention: f64,
) -> Result<f64, &'static str> {
    if !(0.0..=1.0).contains(&cumulative_signal_retention) {
        return Err("alpha_bar вне [0,1]");
    }
    Ok(cumulative_signal_retention.sqrt() * clean
        + (1.0 - cumulative_signal_retention).sqrt() * epsilon)
}
