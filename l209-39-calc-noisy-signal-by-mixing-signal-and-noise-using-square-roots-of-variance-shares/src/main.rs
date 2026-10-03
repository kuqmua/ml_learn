// Урок 39.1. Зашумлённый сигнал: смешивание сигнала и шума с весами из корней долей их разброса.
// Зачем здесь эта тема: Диффузионная модель учится обращать управляемое зашумление данных.
// Почему код устроен так: Смешиваем исходный сигнал с известным шумом по коэффициенту, сохраняя обе
//   составляющие.
// Представь: К исходному числу добавляем известный шум с заданным весом и получаем зашумлённое
//   число.
// При уменьшении доли исходного сигнала смесь становится ближе к шуму.

use l209_39_calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares::calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares;

fn main() {
    let noise: f64 = -1.0;
    let clean: f64 = 2.0;
    for original_signal_variance_share in [1.0, 0.75, 0.25, 0.0] {
        let _: f64 =
            calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
                clean,
                noise,
                original_signal_variance_share,
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

    // Выполняем вычисления из примера.
    let _ = (clean, noise);
}
