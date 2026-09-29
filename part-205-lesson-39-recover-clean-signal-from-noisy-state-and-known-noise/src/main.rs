// Урок 39.3. Восстановление исходного сигнала из зашумлённого состояния и известного шума.
// Зачем здесь эта тема: Если оценка шума известна, из зашумлённого состояния можно приблизить
//   исходный сигнал.
// Почему код устроен так: Алгебраически обращаем формулу прямого смешивания и проверяем
//   восстановление на известных числах.
// Представь: Если известны зашумлённое значение и добавленный шум, исходное можно восстановить
//   обратной формулой.
// При известном точном шуме можно алгебраически восстановить x_0.

// Долю (fraction) дисперсии исходного сигнала обозначают alpha_bar; её сохранение называют retention.
fn reconstruct_clean_signal_from_noisy_sample(
    noisy: f64,
    predicted_noise: f64,
    original_signal_variance_share: f64,
) -> f64 {
    (noisy - (1.0 - original_signal_variance_share).sqrt() * predicted_noise)
        / original_signal_variance_share.sqrt()
}
fn main() {
    lesson_trace::enable();
    let clean: f64 = 2.0;
    lesson_trace::trace_step!(clean);
    let noise: f64 = -0.7;
    lesson_trace::trace_step!(noise);
    let original_signal_variance_share: f64 = 0.36;
    lesson_trace::trace_step!(original_signal_variance_share);
    let noisy: f64 =
        part_203_lesson_39_mix_clean_signal_with_noise_in_forward_diffusion::add_scaled_noise_to_clean_signal_for_diffusion_step(
            clean,
            noise,
            original_signal_variance_share,
        )
        .unwrap();
    lesson_trace::trace_step!(noisy);
    let exact: f64 =
        reconstruct_clean_signal_from_noisy_sample(noisy, noise, original_signal_variance_share);
    lesson_trace::trace_step!(exact);
    let mistaken: f64 = reconstruct_clean_signal_from_noisy_sample(
        noisy,
        noise + 0.2,
        original_signal_variance_share,
    );
    lesson_trace::trace_step!(mistaken);
    assert!((exact - clean).abs() < 1e-12);
    assert!((mistaken - clean).abs() > 0.1);
    println!("x_t={noisy:.3}; x_0 при точном шуме={exact:.3}; при ошибке={mistaken:.3}");
    // На практике сеть предсказывает шум по x_t и t; истинный шум при генерации неизвестен.
}
