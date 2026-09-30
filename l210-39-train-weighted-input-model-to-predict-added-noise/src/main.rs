// Урок 39.2. Обучение модели взвешенного входа прогнозу добавленного шума.
// Связь с принятой терминологией: Обучение линейной модели предсказанию шума в зашумлённом сигнале.
// Зачем здесь эта тема: Чтобы убрать неизвестный шум, модель сначала должна научиться его
//   предсказывать по зашумлённому входу.
// Почему код устроен так: Обучаем простую линейную оценку шума на парах из прямого шага.
// Представь: На учебных парах модель видит зашумлённый вход и правильный добавленный шум.
// На синтетической паре учим линейный предсказатель epsilon по x_t и исходному условию.

use l209_39_calculate_noisy_signal_by_mixing_signal_and_noise_with_root_variance_share_weights::calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares;

fn main() {
    let alpha: f64 = 0.64;
    let training: [(f64, f64); 4] = [(1.0, -1.0), (1.0, 0.0), (1.0, 1.0), (1.0, 2.0)];
    let validation: [(f64, f64); 2] = [(2.0, -0.5), (-1.0, 0.5)];
    let inputs: [(f64, f64); 4] = training.map(|(clean, noise)| {
        (
            calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
                clean, noise, alpha,
            )
            .unwrap() - alpha.sqrt() * clean,
            noise,
        )
    });
    let mut weight: f64 = 0.0;
    for _ in 0..100 {
        let rate_of_change: f64 = inputs
            .iter()
            .map(|&(input_value, target)| 2.0 * (weight * input_value - target) * input_value)
            .sum::<f64>()
            / inputs.len() as f64;
        weight -= 0.2 * rate_of_change;
    }
    let loss: f64 = inputs
        .iter()
        .map(|&(input_value, target)| (weight * input_value - target).powi(2))
        .sum::<f64>()
        / inputs.len() as f64;
    assert!(loss < 1e-6);
    let held_out: [(f64, f64); 2] = validation.map(|(clean, noise)| {
        (
            calculate_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
                clean, noise, alpha,
            )
            .unwrap() - alpha.sqrt() * clean,
            noise,
        )
    });
    let mean_squared_error_value: &dyn Fn(f64) -> f64 = &|candidate: f64| {
        held_out
            .iter()
            .map(|&(input_value, target)| (candidate * input_value - target).powi(2))
            .sum::<f64>()
            / held_out.len() as f64
    };
    let baseline: f64 = mean_squared_error_value(0.0);
    let validation_loss: f64 = mean_squared_error_value(weight);
    assert!(validation_loss < baseline);
}
