// Урок 39.2. Обучение линейной модели предсказанию шума в зашумлённом сигнале.
// На синтетической паре учим линейный предсказатель epsilon по x_t и исходному условию.

fn main() {
    let alpha: f64 = 0.64;
    let training: [(f64, f64); 4] = [(1.0, -1.0), (1.0, 0.0), (1.0, 1.0), (1.0, 2.0)];
    let validation: [(f64, f64); 2] = [(2.0, -0.5), (-1.0, 0.5)];
    // Условный предсказатель получает известное clean: пример изолирует MSE обучения.
    let inputs: Vec<(f64, f64)> = training
        .iter()
        .map(|&(clean, noise)| {
            (
                part_203_lesson_39_mix_clean_signal_with_noise_in_forward_diffusion::add_scaled_noise_to_clean_signal_for_diffusion_step(
                    clean, noise, alpha,
                )
                .unwrap()
                    - alpha.sqrt() * clean,
                noise,
            )
        })
        .collect();
    let mut weight: f64 = 0.0;
    for _ in 0..100 {
        // Производную функции по параметру или вектор таких производных называют gradient.
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
    // Отложенные пары не участвовали в изменении веса.
    let held_out: Vec<(f64, f64)> = validation
        .iter()
        .map(|&(clean, noise)| {
            (
                part_203_lesson_39_mix_clean_signal_with_noise_in_forward_diffusion::add_scaled_noise_to_clean_signal_for_diffusion_step(
                    clean, noise, alpha,
                )
                .unwrap()
                    - alpha.sqrt() * clean,
                noise,
            )
        })
        .collect();
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
    println!(
        "вес={weight:.3}; train MSE={loss:.8}; validation MSE={validation_loss:.8}; baseline={baseline:.3}"
    );
    // В реальной модели clean при генерации неизвестен; это только проверка loss и градиента.
}
