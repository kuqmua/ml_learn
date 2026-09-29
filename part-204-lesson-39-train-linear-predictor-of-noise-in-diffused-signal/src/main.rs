// Урок 39.2. Обучение линейной модели предсказанию шума в зашумлённом сигнале.
// Почему этот урок сейчас: Чтобы убрать неизвестный шум, модель сначала должна научиться его предсказывать по зашумлённому входу.
// Почему пример устроен так: Обучаем простую линейную оценку шума на парах из прямого шага.
// На синтетической паре учим линейный предсказатель epsilon по x_t и исходному условию.

fn main() {
    lesson_trace::enable();
    // α=0.64 сохраняет 64% дисперсии чистого сигнала; оставшиеся 36% приходятся на шум.
    let alpha: f64 = 0.64;
    lesson_trace::trace_step!(alpha);
    let training: [(f64, f64); 4] = [(1.0, -1.0), (1.0, 0.0), (1.0, 1.0), (1.0, 2.0)];
    lesson_trace::trace_step!(training);
    let validation: [(f64, f64); 2] = [(2.0, -0.5), (-1.0, 0.5)];
    lesson_trace::trace_step!(validation);
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
    lesson_trace::trace_step!(inputs);
    let mut weight: f64 = 0.0;
    lesson_trace::trace_step!(weight);
    // 100 шагов градиентного спуска подгоняют один вес к четырём обучающим парам.
    // Множитель 0.2 ниже — выбранная скорость обучения, то есть доля градиента за шаг.
    for _ in 0..100 {
        // Производную функции по параметру или вектор таких производных называют gradient.
        let rate_of_change: f64 = inputs
            .iter()
            .map(|&(input_value, target)| 2.0 * (weight * input_value - target) * input_value)
            .sum::<f64>()
            / inputs.len() as f64;
        lesson_trace::trace_step!(rate_of_change);
        weight -= 0.2 * rate_of_change;
        lesson_trace::trace_step!(weight);
    }
    let loss: f64 = inputs
        .iter()
        .map(|&(input_value, target)| (weight * input_value - target).powi(2))
        .sum::<f64>()
        / inputs.len() as f64;
    lesson_trace::trace_step!(loss);
    // Требуем MSE ниже 10⁻⁶: это проверка, что один вес действительно подогнал учебные пары.
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
    lesson_trace::trace_step!(held_out);
    let mean_squared_error_value: &dyn Fn(f64) -> f64 = &|candidate: f64| {
        held_out
            .iter()
            .map(|&(input_value, target)| (candidate * input_value - target).powi(2))
            .sum::<f64>()
            / held_out.len() as f64
    };
    let baseline: f64 = mean_squared_error_value(0.0);
    lesson_trace::trace_step!(baseline);
    let validation_loss: f64 = mean_squared_error_value(weight);
    lesson_trace::trace_step!(validation_loss);
    assert!(validation_loss < baseline);
    println!(
        "вес={weight:.3}; train MSE={loss:.8}; validation MSE={validation_loss:.8}; baseline={baseline:.3}"
    );
    // В реальной модели clean при генерации неизвестен; это только проверка loss и градиента.
}
