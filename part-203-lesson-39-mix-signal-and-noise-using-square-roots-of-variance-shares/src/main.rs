// Урок 39.1. Смешивание сигнала и шума с весами из корней их долей в разбросе.
// Связь с принятой терминологией: Смешивание исходного сигнала с шумом при прямой диффузии.
// Зачем здесь эта тема: Диффузионная модель учится обращать управляемое зашумление данных.
// Почему код устроен так: Смешиваем исходный сигнал с известным шумом по коэффициенту, сохраняя обе
//   составляющие.
// Представь: К исходному числу добавляем известный шум с заданным весом и получаем зашумлённое
//   число.
// При уменьшении доли исходного сигнала смесь становится ближе к шуму.

fn main() {
    lesson_trace::enable();
    let clean: f64 = 2.0;
    lesson_trace::trace_step!(clean);
    let noise: f64 = -1.0;
    lesson_trace::trace_step!(noise);
    // Долю (fraction) дисперсии исходного сигнала обозначают alpha_bar; её сохранение называют retention.
    for original_signal_variance_share in [1.0, 0.75, 0.25, 0.0] {
        lesson_trace::trace_step!(original_signal_variance_share);
        let noisy: f64 = part_203_lesson_39_mix_signal_and_noise_using_square_roots_of_variance_shares::calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean,
            noise,
            original_signal_variance_share,
        )
        .unwrap();
        lesson_trace::trace_step!(noisy);
        println!("alpha_bar={original_signal_variance_share:.2}; x_t={noisy:.3}");
    }
    assert_eq!(
        part_203_lesson_39_mix_signal_and_noise_using_square_roots_of_variance_shares::calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean, noise, 1.0
        )
        .unwrap(),
        clean
    );
    assert_eq!(
        part_203_lesson_39_mix_signal_and_noise_using_square_roots_of_variance_shares::calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean, noise, 0.0
        )
        .unwrap(),
        noise
    );
    lesson_trace::disable();
    plot_signal_and_noise_mixture_for_changing_signal_share(clean, noise);
}

fn plot_signal_and_noise_mixture_for_changing_signal_share(clean: f64, noise: f64) {
    let points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            let original_signal_variance_share: f64 = plot_step_index as f64 / 100.0;
            (
                original_signal_variance_share,
                part_203_lesson_39_mix_signal_and_noise_using_square_roots_of_variance_shares::calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
                    clean,
                    noise,
                    original_signal_variance_share,
                )
                .unwrap(),
            )
        })
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "diffusion-forward",
        "Смесь сигнала и шума",
        "alpha_bar",
        "x_t",
        &[lesson_visualization::Series {
            name: "x_t",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
