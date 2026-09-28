//! Прямой процесс диффузии.

/// Прямой шаг диффузии при заранее выбранном шуме epsilon.
pub fn add_noise(clean: f64, epsilon: f64, alpha_bar: f64) -> Result<f64, &'static str> {
    if !(0.0..=1.0).contains(&alpha_bar) {
        return Err("alpha_bar вне [0,1]");
    }
    Ok(alpha_bar.sqrt() * clean + (1.0 - alpha_bar).sqrt() * epsilon)
}
