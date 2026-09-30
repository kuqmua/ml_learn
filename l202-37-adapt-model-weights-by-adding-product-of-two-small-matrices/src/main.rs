// Урок 37.1. Адаптация весов модели: прибавление произведения двух небольших матриц к исходным весам.
// Связь с принятой терминологией: Низкоранговая адаптация замороженной матрицы весов.
// Зачем здесь эта тема: Полная перенастройка большой модели требует хранить и обновлять много
//   весов.
// Почему код устроен так: Замораживаем исходную матрицу и обучаем две маленькие матрицы,
//   произведение которых задаёт поправку.
// Представь: Вместо изменения большой матрицы добавляем произведение двух маленьких матриц к её
//   выходу.
// Замороженную матрицу дополняют произведением маленьких обучаемых матриц.

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    let frozen: [[f64; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    trace_step!(frozen);
    trace_note!("Rank 1: A имеет форму 4x1, B — 1x4.");
    let adapter_output_weights: [f64; 4] = [0.1, 0.2, 0.3, 0.4];
    trace_step!(adapter_output_weights);
    let adapter_input_weights: [f64; 4] = [1.0, 0.0, -1.0, 0.0];
    trace_step!(adapter_input_weights);
    let input: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
    trace_step!(input);
    let projected_input: f64 = adapter_input_weights
        .iter()
        .zip(input)
        .map(|(adapter_component, input_component)| adapter_component * input_component)
        .sum();
    trace_step!(projected_input);
    let output: [f64; 4] = std::array::from_fn(|row| {
        frozen[row]
            .iter()
            .zip(input)
            .map(|(weight_value, input_component)| weight_value * input_component)
            .sum::<f64>()
            + adapter_output_weights[row] * projected_input
    });
    trace_step!(output);
    assert_eq!(output[0], 0.8);
    println!("выход с LoRA: {output:?}; trainable=8 вместо 16 параметров");
}
