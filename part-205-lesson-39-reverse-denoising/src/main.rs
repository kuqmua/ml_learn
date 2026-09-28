// Урок 39.3. Обратное восстановление из шумного состояния.
// При известном точном шуме можно алгебраически восстановить x_0.

use part_203_lesson_39_diffusion_forward::add_noise;
// Долю (fraction) дисперсии исходного сигнала обозначают alpha_bar; её сохранение называют retention.
fn reconstruct(noisy: f64, predicted_noise: f64, original_signal_variance_share: f64) -> f64 {
    (noisy - (1.0 - original_signal_variance_share).sqrt() * predicted_noise)
        / original_signal_variance_share.sqrt()
}
fn main() {
    let clean = 2.0;
    let noise = -0.7;
    let original_signal_variance_share = 0.36;
    let noisy = add_noise(clean, noise, original_signal_variance_share).unwrap();
    let exact = reconstruct(noisy, noise, original_signal_variance_share);
    let mistaken = reconstruct(noisy, noise + 0.2, original_signal_variance_share);
    assert!((exact - clean).abs() < 1e-12);
    assert!((mistaken - clean).abs() > 0.1);
    println!("x_t={noisy:.3}; x_0 при точном шуме={exact:.3}; при ошибке={mistaken:.3}");
    // На практике сеть предсказывает шум по x_t и t; истинный шум при генерации неизвестен.
}
