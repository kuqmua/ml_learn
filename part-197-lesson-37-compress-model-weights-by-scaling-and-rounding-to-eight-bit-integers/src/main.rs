// Урок 37.2. Сжатие весов модели: масштабирование и округление до восьмибитных целых чисел.
// Связь с принятой терминологией: Симметричное квантование весов модели в INT8.
// Зачем здесь эта тема: Большие веса занимают память и замедляют перенос модели.
// Почему код устроен так: Отображаем вещественные веса в INT8 с масштабом и измеряем ошибку
//   восстановления.
// Представь: Число 0,25 хранится как целый код и масштаб; после восстановления оно может немного
//   отличаться.
// Масштабируем веса в i8 и измеряем погрешность после восстановления.

fn main() {
    lesson_trace::enable();
    let weights: [f64; 5] = [-1.0, -0.5, 0.0, 0.25, 1.0];
    lesson_trace::trace_step!(weights);
    let maximum: f64 = weights.iter().copied().map(f64::abs).fold(0.0, f64::max);
    lesson_trace::trace_step!(maximum);
    // 127 — наибольший положительный i8: масштаб переводит максимальный |вес| в код ±127.
    let scale: f64 = maximum / 127.0;
    lesson_trace::trace_step!(scale);
    // Представление весов целыми числами меньшей точности называют quantization.
    let reduced_precision_weights: Vec<i8> = weights
        .iter()
        // Симметричный диапазон −127..127 оставляет ноль точным и не использует лишний код −128.
        .map(|&weight_value| (weight_value / scale).round().clamp(-127.0, 127.0) as i8)
        .collect();
    lesson_trace::trace_step!(reduced_precision_weights);
    let reconstructed: Vec<f64> = reduced_precision_weights
        .iter()
        .map(|&reduced_precision_weight| f64::from(reduced_precision_weight) * scale)
        .collect();
    lesson_trace::trace_step!(reconstructed);
    let error: f64 = weights
        .iter()
        .zip(&reconstructed)
        .map(|(first_value, second_value)| (first_value - second_value).abs())
        .fold(0.0, f64::max);
    lesson_trace::trace_step!(error);
    assert!(error <= scale / 2.0 + 1e-12);
    println!("INT8={reduced_precision_weights:?}; максимум ошибки={error:.6}");
    // Это per-tensor учебный пример, не алгоритм LLM.int8() с обработкой выбросов.
}
