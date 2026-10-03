//! Урок 209. Зашумлённый сигнал: смешивание сигнала и шума с весами из корней долей их разброса.

/// Прямой шаг диффузии при заранее выбранном шуме epsilon.
/// Прямой шаг диффузии: sqrt(a)·signal + sqrt(1−a)·noise; масштабируем и сигнал, и шум.

pub fn calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
    clean: f64,
    sampled_noise_value: f64,

    original_signal_variance_share: f64,
) -> Result<f64, &'static str> {
    if !(0.0..=1.0).contains(&original_signal_variance_share) {
        return Err("доля исходного сигнала должна быть числом от 0 до 1");
    }
    Ok(original_signal_variance_share.sqrt() * clean
        + (1.0 - original_signal_variance_share).sqrt() * sampled_noise_value)
}
