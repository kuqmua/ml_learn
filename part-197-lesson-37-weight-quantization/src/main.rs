// Урок 37.2. Симметричное INT8 квантование.
// Масштабируем веса в i8 и измеряем погрешность после восстановления.

fn main() {
    let weights = [-1.0, -0.5, 0.0, 0.25, 1.0];
    let maximum = weights.iter().copied().map(f64::abs).fold(0.0, f64::max);
    let scale = maximum / 127.0;
    // Представление весов целыми числами меньшей точности называют quantization.
    let reduced_precision_weights: Vec<i8> = weights
        .iter()
        .map(|&weight_value| (weight_value / scale).round().clamp(-127.0, 127.0) as i8)
        .collect();
    let reconstructed: Vec<f64> = reduced_precision_weights
        .iter()
        .map(|&reduced_precision_weight| f64::from(reduced_precision_weight) * scale)
        .collect();
    let error = weights
        .iter()
        .zip(&reconstructed)
        .map(|(first_value, second_value)| (first_value - second_value).abs())
        .fold(0.0, f64::max);
    assert!(error <= scale / 2.0 + 1e-12);
    println!("INT8={reduced_precision_weights:?}; максимум ошибки={error:.6}");
    // Это per-tensor учебный пример, не алгоритм LLM.int8() с обработкой выбросов.
}
