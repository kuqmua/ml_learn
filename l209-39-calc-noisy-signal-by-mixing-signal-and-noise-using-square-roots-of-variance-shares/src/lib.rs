//! Урок 209. Зашумлённый сигнал: смешивание сигнала и шума с весами из корней долей их разброса.

/// Прямой шаг диффузии при заранее выбранном шуме epsilon.
/// Прямой шаг диффузии: sqrt(a)·signal + sqrt(1−a)·noise; масштабируем и сигнал, и шум.

pub fn calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
    clean: f64,
    sampled_noise_value: f64,

    original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal: f64,
) -> Result<f64, &'static str> {
    if !(0.0..=1.0)
        .contains(&original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal)
    {
        return Err("доля исходного сигнала должна быть числом от 0 до 1");
    }
    Ok(
        original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal.sqrt()
            * clean
            + (1.0
                - original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal)
                .sqrt()
                * sampled_noise_value,
    )
}
