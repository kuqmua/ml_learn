// Урок 39.3. Восстановленный сигнал: вычитание взвешенного шума и деление на масштаб исходного сигнала.
// Связь с принятой терминологией: Восстановление исходного сигнала из зашумлённого состояния и известного шума.
// Зачем здесь эта тема: Если оценка шума известна, из зашумлённого состояния можно приблизить
//   исходный сигнал.
// Почему код устроен так: Алгебраически обращаем формулу прямого смешивания и проверяем
//   восстановление на известных числах.
// Представь: Если известны зашумлённое значение и добавленный шум, исходное можно восстановить
//   обратной формулой.
// При известном точном шуме можно алгебраически восстановить x_0.

// Долю (fraction) дисперсии исходного сигнала обозначают alpha_bar; её сохранение называют retention.
/// Восстановление сигнала: (noisy − sqrt(1−a)·predicted_noise) / sqrt(a).
use l209_39_calculate_noisy_signal_by_mixing_signal_and_noise_with_root_variance_share_weights::calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares;

fn recover_clean_signal_by_subtracting_scaled_noise_then_dividing_by_signal_scale(
    noisy: f64,
    predicted_noise: f64,
    original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal: f64,
) -> f64 {
    (noisy
        - (1.0 - original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal)
            .sqrt()
            * predicted_noise)
        / original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal.sqrt()
}
fn main() {
    let noise: f64 = -0.7;
    let original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal: f64 =
        0.36;
    let clean: f64 = 2.0;
    let noisy: f64 =
        calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean,
            noise,
            original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal,
        )
        .unwrap();

    assert!(
        (recover_clean_signal_by_subtracting_scaled_noise_then_dividing_by_signal_scale(
            noisy,
            noise,
            original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal,
        ) - clean)
            .abs()
            < 1e-12
    );
    assert!(
        (recover_clean_signal_by_subtracting_scaled_noise_then_dividing_by_signal_scale(
            noisy,
            noise + 0.2,
            original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal,
        ) - clean)
            .abs()
            > 0.1
    );
}
