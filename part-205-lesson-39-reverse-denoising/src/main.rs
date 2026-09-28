// Урок 39.3. Обратное восстановление из шумного состояния.
// При известном точном шуме можно алгебраически восстановить x_0.

use part_203_lesson_39_diffusion_forward::add_noise;
fn reconstruct(noisy: f64, predicted_noise: f64, cumulative_signal_retention: f64) -> f64 {
    (noisy - (1.0 - cumulative_signal_retention).sqrt() * predicted_noise)
        / cumulative_signal_retention.sqrt()
}
fn main() {
    let clean = 2.0;
    let noise = -0.7;
    let alpha = 0.36;
    let noisy = add_noise(clean, noise, alpha).unwrap();
    let exact = reconstruct(noisy, noise, alpha);
    let mistaken = reconstruct(noisy, noise + 0.2, alpha);
    assert!((exact - clean).abs() < 1e-12);
    assert!((mistaken - clean).abs() > 0.1);
    println!("x_t={noisy:.3}; x_0 при точном шуме={exact:.3}; при ошибке={mistaken:.3}");
    // На практике сеть предсказывает шум по x_t и t; истинный шум при генерации неизвестен.
}
