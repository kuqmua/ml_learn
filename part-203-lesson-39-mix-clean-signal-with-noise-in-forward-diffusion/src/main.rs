// Урок 39.1. Смешивание исходного сигнала с шумом при прямой диффузии.
// При уменьшении доли исходного сигнала смесь становится ближе к шуму.

fn main() {
    let clean = 2.0;
    let noise = -1.0;
    // Долю (fraction) дисперсии исходного сигнала обозначают alpha_bar; её сохранение называют retention.
    for original_signal_variance_share in [1.0, 0.75, 0.25, 0.0] {
        let noisy = part_203_lesson_39_mix_clean_signal_with_noise_in_forward_diffusion::add_scaled_noise_to_clean_signal_for_diffusion_step(
            clean,
            noise,
            original_signal_variance_share,
        )
        .unwrap();
        println!("alpha_bar={original_signal_variance_share:.2}; x_t={noisy:.3}");
    }
    assert_eq!(
        part_203_lesson_39_mix_clean_signal_with_noise_in_forward_diffusion::add_scaled_noise_to_clean_signal_for_diffusion_step(
            clean, noise, 1.0
        )
        .unwrap(),
        clean
    );
    assert_eq!(
        part_203_lesson_39_mix_clean_signal_with_noise_in_forward_diffusion::add_scaled_noise_to_clean_signal_for_diffusion_step(
            clean, noise, 0.0
        )
        .unwrap(),
        noise
    );
    visualize_mix_clean_signal_with_noise_in_forward_diffusion(clean, noise);
}

fn visualize_mix_clean_signal_with_noise_in_forward_diffusion(clean: f64, noise: f64) {
    let points: Vec<_> = (0..=100)
        .map(|plot_step_index| {
            let original_signal_variance_share = plot_step_index as f64 / 100.0;
            (
                original_signal_variance_share,
                part_203_lesson_39_mix_clean_signal_with_noise_in_forward_diffusion::add_scaled_noise_to_clean_signal_for_diffusion_step(
                    clean,
                    noise,
                    original_signal_variance_share,
                )
                .unwrap(),
            )
        })
        .collect();
    let path = lesson_visualization::line_chart(
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
