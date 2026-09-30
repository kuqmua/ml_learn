// Урок 37.2. Сжатие весов модели: масштабирование и округление до восьмибитных целых чисел.
// Связь с принятой терминологией: Симметричное квантование весов модели в INT8.
// Зачем здесь эта тема: Большие веса занимают память и замедляют перенос модели.
// Почему код устроен так: Отображаем вещественные веса в INT8 с масштабом и измеряем ошибку
//   восстановления.
// Представь: Число 0,25 хранится как целый код и масштаб; после восстановления оно может немного
//   отличаться.
// Масштабируем веса в i8 и измеряем погрешность после восстановления.

fn main() {
    let weights: [f64; 5] = [-1.0, -0.5, 0.0, 0.25, 1.0];
    let maximum: f64 = weights.iter().copied().map(f64::abs).fold(0.0, f64::max);
    let scale: f64 = maximum / 127.0;
    let reduced_precision_weights: [i8; 5] =
        weights.map(|weight_value| (weight_value / scale).round().clamp(-127.0, 127.0) as i8);
    let reconstructed: [f64; 5] = reduced_precision_weights
        .map(|reduced_precision_weight| f64::from(reduced_precision_weight) * scale);
    let error: f64 = weights
        .iter()
        .zip(&reconstructed)
        .map(|(first_value, second_value)| (first_value - second_value).abs())
        .fold(0.0, f64::max);
    assert!(error <= scale / 2.0 + 1e-12);
}
