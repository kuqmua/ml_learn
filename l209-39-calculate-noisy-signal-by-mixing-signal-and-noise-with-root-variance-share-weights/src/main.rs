// Урок 39.1. Зашумлённый сигнал: смешивание сигнала и шума с весами из корней долей их разброса.
// Связь с принятой терминологией: Смешивание исходного сигнала с шумом при прямой диффузии.
// Зачем здесь эта тема: Диффузионная модель учится обращать управляемое зашумление данных.
// Почему код устроен так: Смешиваем исходный сигнал с известным шумом по коэффициенту, сохраняя обе
//   составляющие.
// Представь: К исходному числу добавляем известный шум с заданным весом и получаем зашумлённое
//   число.
// При уменьшении доли исходного сигнала смесь становится ближе к шуму.

use l209_39_calculate_noisy_signal_by_mixing_signal_and_noise_with_root_variance_share_weights::calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares;

fn main() {
    let noise: f64 = -1.0;
    let clean: f64 = 2.0;
    for original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal in
        [1.0, 0.75, 0.25, 0.0]
    {
        let _: f64 =
            calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
                clean,
                noise,
                original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal,
            )
            .unwrap();
    }
    assert_eq!(
        calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean, noise, 1.0
        )
        .unwrap(),
        clean
    );
    assert_eq!(
        calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean, noise, 0.0
        )
        .unwrap(),
        noise
    );
    plot_signal_and_noise_mixture_for_changing_signal_share(clean, noise);
}

fn plot_signal_and_noise_mixture_for_changing_signal_share(clean: f64, noise: f64) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "diffusion-forward",
        "Смесь сигнала и шума",
        "alpha_bar",
        "x_t",
        &[lesson_visualization::Series {
            name: "x_t",
            points: &(0..=100)
        .map(|plot_step_index| {
            let original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal: f64 = plot_step_index as f64 / 100.0;
            (
                original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal,
                calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
                    clean,
                    noise,
                    original_signal_variance_share_where_0_means_only_noise_and_1_means_clean_signal,
                )
                .unwrap(),
            )
        })
        .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
