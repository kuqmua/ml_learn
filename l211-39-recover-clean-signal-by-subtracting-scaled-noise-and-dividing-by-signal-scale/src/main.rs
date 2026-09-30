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

use lesson_trace::{enable_tracing, trace_note, trace_step};

fn recover_clean_signal_by_subtracting_scaled_noise_then_dividing_by_signal_scale(
    noisy: f64,
    predicted_noise: f64,
    original_signal_variance_share: f64,
) -> f64 {
    (noisy - (1.0 - original_signal_variance_share).sqrt() * predicted_noise)
        / original_signal_variance_share.sqrt()
}
fn main() {
    enable_tracing();
    let clean: f64 = 2.0;
    trace_step!(clean);
    let noise: f64 = -0.7;
    trace_step!(noise);
    let original_signal_variance_share: f64 = 0.36;
    trace_step!(original_signal_variance_share);
    let noisy: f64 =
        calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean,
            noise,
            original_signal_variance_share,
        )
        .unwrap();
    trace_step!(noisy);
    let exact: f64 = recover_clean_signal_by_subtracting_scaled_noise_then_dividing_by_signal_scale(
        noisy,
        noise,
        original_signal_variance_share,
    );
    trace_step!(exact);
    let mistaken: f64 =
        recover_clean_signal_by_subtracting_scaled_noise_then_dividing_by_signal_scale(
            noisy,
            noise + 0.2,
            original_signal_variance_share,
        );
    trace_step!(mistaken);
    assert!((exact - clean).abs() < 1e-12);
    assert!((mistaken - clean).abs() > 0.1);
    println!("x_t={noisy:.3}; x_0 при точном шуме={exact:.3}; при ошибке={mistaken:.3}");
    trace_note!(
        "На практике сеть предсказывает шум по x_t и t; истинный шум при генерации неизвестен."
    );
}
