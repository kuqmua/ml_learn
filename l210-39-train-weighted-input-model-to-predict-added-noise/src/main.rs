// Урок 210. Обучаем предсказатель шума по одному зашумлённому значению.
// Чистый сигнал нужен для создания учебных пар, но не передаётся модели при прогнозе.
use l209_39_calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares::calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares;

fn main() {
    let mix = |clean, noise| {
        calc_noisy_signal_by_mixing_signal_and_noise_using_square_roots_of_variance_shares(
            clean, noise, 0.64,
        )
        .unwrap()
    };
    let mut training = Vec::new();
    for clean in [-1.0, 0.0, 1.0] {
        for noise in [-1.0, 0.0, 1.0] {
            training.push((mix(clean, noise), noise));
        }
    }
    let mut weight = 0.0;
    for _ in 0..200 {
        let derivative = training
            .iter()
            .map(|&(noisy, noise)| 2.0 * (weight * noisy - noise) * noisy)
            .sum::<f64>()
            / training.len() as f64;
        weight -= 0.1 * derivative;
    }
    // Прогноз принимает только noisy: исходный clean и правильный noise недоступны.
    let predict_noise = |noisy: f64| weight * noisy;
    let mut model_error = 0.0;
    let mut baseline_error = 0.0;
    for clean in [-0.5, 0.5] {
        for noise in [-0.5, 0.5] {
            let noisy = mix(clean, noise);
            let predicted = predict_noise(noisy);
            model_error += (predicted - noise).powi(2) / 4.0;
            baseline_error += noise.powi(2) / 4.0;
            println!("Зашумлённый вход={noisy}: прогноз шума={predicted}, правильный шум={noise}");
        }
    }
    assert!(model_error < baseline_error);
    assert!(model_error > 0.0);
    println!(
        "Вес={weight}; ошибка на новых парах={model_error}, нулевого прогноза={baseline_error}"
    );
    // Один наблюдаемый вход может иметь разные объяснения: 0.8*clean + 0.6*noise.
    // Для clean=0.75, noise=-1 и clean=-0.75, noise=1 получаем около нуля.
    let input1 = mix(0.75, -1.0);
    let input2 = mix(-0.75, 1.0);
    println!(
        "Разный шум -1 и 1 даёт почти одинаковые входы {input1:e} и {input2:e}; точно угадать его только по входу нельзя."
    );
}

// Чему учит этот урок:
// Строим пары зашумлённого значения и шума и обучаем линейный прогноз без доступа к чистому сигналу.
// Сравниваем с нулевым прогнозом на новых парах; улучшение не означает точного восстановления.
// Это маленький предсказатель при фиксированной силе шума, а не полная диффузионная модель.
