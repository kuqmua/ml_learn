//! Прямой процесс диффузии.

/// Прямой шаг диффузии при заранее выбранном шуме epsilon.
pub fn add_noise(
    clean: f64,
    epsilon: f64,
    // Долю (fraction) дисперсии исходного сигнала обозначают alpha_bar; её сохранение называют retention.
    original_signal_variance_share: f64,
) -> Result<f64, &'static str> {
    if !(0.0..=1.0).contains(&original_signal_variance_share) {
        return Err("alpha_bar вне [0,1]");
    }
    Ok(original_signal_variance_share.sqrt() * clean
        + (1.0 - original_signal_variance_share).sqrt() * epsilon)
}
