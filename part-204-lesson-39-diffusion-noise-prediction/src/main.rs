// Урок 39.2. Обучение предсказателя шума.
// На синтетической паре учим линейный предсказатель epsilon по x_t и исходному условию.

use part_203_lesson_39_diffusion_forward::add_noise;
fn main() {
    let alpha: f64 = 0.64;
    let training = [(1.0, -1.0), (1.0, 0.0), (1.0, 1.0), (1.0, 2.0)];
    let validation = [(2.0, -0.5), (-1.0, 0.5)];
    // Условный предсказатель получает известное clean: пример изолирует MSE обучения.
    let inputs: Vec<_> = training
        .iter()
        .map(|&(clean, noise)| {
            (
                add_noise(clean, noise, alpha).unwrap() - alpha.sqrt() * clean,
                noise,
            )
        })
        .collect();
    let mut weight = 0.0;
    for _ in 0..100 {
        let gradient = inputs
            .iter()
            .map(|&(x, target)| 2.0 * (weight * x - target) * x)
            .sum::<f64>()
            / inputs.len() as f64;
        weight -= 0.2 * gradient;
    }
    let loss = inputs
        .iter()
        .map(|&(x, target)| (weight * x - target).powi(2))
        .sum::<f64>()
        / inputs.len() as f64;
    assert!(loss < 1e-6);
    // Отложенные пары не участвовали в изменении веса.
    let held_out: Vec<_> = validation
        .iter()
        .map(|&(clean, noise)| {
            (
                add_noise(clean, noise, alpha).unwrap() - alpha.sqrt() * clean,
                noise,
            )
        })
        .collect();
    let mse = |candidate: f64| {
        held_out
            .iter()
            .map(|&(x, target)| (candidate * x - target).powi(2))
            .sum::<f64>()
            / held_out.len() as f64
    };
    let baseline = mse(0.0);
    let validation_loss = mse(weight);
    assert!(validation_loss < baseline);
    println!(
        "вес={weight:.3}; train MSE={loss:.8}; validation MSE={validation_loss:.8}; baseline={baseline:.3}"
    );
    // В реальной модели clean при генерации неизвестен; это только проверка loss и градиента.
}
