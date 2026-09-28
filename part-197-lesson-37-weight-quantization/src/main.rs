// Урок 37.2. Симметричное INT8 квантование.
// Масштабируем веса в i8 и измеряем погрешность после восстановления.

fn main() {
    let weights = [-1.0, -0.5, 0.0, 0.25, 1.0];
    let maximum = weights.iter().copied().map(f64::abs).fold(0.0, f64::max);
    let scale = maximum / 127.0;
    let quantized: Vec<i8> = weights
        .iter()
        .map(|&w| (w / scale).round().clamp(-127.0, 127.0) as i8)
        .collect();
    let reconstructed: Vec<f64> = quantized.iter().map(|&q| f64::from(q) * scale).collect();
    let error = weights
        .iter()
        .zip(&reconstructed)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max);
    assert!(error <= scale / 2.0 + 1e-12);
    println!("INT8={quantized:?}; максимум ошибки={error:.6}");
    // Это per-tensor учебный пример, не алгоритм LLM.int8() с обработкой выбросов.
}
