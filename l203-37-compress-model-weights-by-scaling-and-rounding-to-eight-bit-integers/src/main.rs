// Урок 37.2. Сжатие весов модели: масштабирование и округление до восьмибитных целых чисел.
// Связь с принятой терминологией: Симметричное квантование весов модели в INT8.
// Зачем здесь эта тема: Большие веса занимают память и замедляют перенос модели.
// Почему код устроен так: Отображаем вещественные веса в INT8 с масштабом и измеряем ошибку
//   восстановления.
// Представь: Число 0,25 хранится как целый код и масштаб; после восстановления оно может немного
//   отличаться.
// Масштабируем веса в i8 и измеряем погрешность после восстановления.

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    let weights: [f64; 5] = [-1.0, -0.5, 0.0, 0.25, 1.0];
    trace_step!(weights);
    let maximum: f64 = weights.iter().copied().map(f64::abs).fold(0.0, f64::max);
    trace_step!(maximum);
    trace_note!(
        "127 — наибольший положительный i8: масштаб переводит максимальный |вес| в код ±127."
    );
    let scale: f64 = maximum / 127.0;
    trace_step!(scale);
    trace_note!("Представление весов целыми числами меньшей точности называют quantization.");
    trace_note!(
        "Симметричный диапазон −127..127 оставляет ноль точным и не использует лишний код −128."
    );
    let reduced_precision_weights: Vec<i8> = weights
        .iter()
        .map(|&weight_value| (weight_value / scale).round().clamp(-127.0, 127.0) as i8)
        .collect();
    trace_step!(reduced_precision_weights);
    let reconstructed: Vec<f64> = reduced_precision_weights
        .iter()
        .map(|&reduced_precision_weight| f64::from(reduced_precision_weight) * scale)
        .collect();
    trace_step!(reconstructed);
    let error: f64 = weights
        .iter()
        .zip(&reconstructed)
        .map(|(first_value, second_value)| (first_value - second_value).abs())
        .fold(0.0, f64::max);
    trace_step!(error);
    assert!(error <= scale / 2.0 + 1e-12);
    println!("INT8={reduced_precision_weights:?}; максимум ошибки={error:.6}");
    trace_note!("Это per-tensor учебный пример, не алгоритм LLM.int8() с обработкой выбросов.");
}
