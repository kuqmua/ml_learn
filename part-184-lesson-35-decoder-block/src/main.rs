// Урок 35.4. Pre-norm decoder block.
// Нормализация, причинное внимание, residual, FFN и второй residual образуют блок.

use part_182_lesson_35_causal_self_attention::causal_attention;
fn layer_norm(input_value: [f64; 2]) -> [f64; 2] {
    let mean = (input_value[0] + input_value[1]) / 2.0;
    let variance = ((input_value[0] - mean).powi(2) + (input_value[1] - mean).powi(2)) / 2.0;
    [
        (input_value[0] - mean) / (variance + 1e-5).sqrt(),
        (input_value[1] - mean) / (variance + 1e-5).sqrt(),
    ]
}
fn block(input: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let normalized: Vec<_> = input.iter().copied().map(layer_norm).collect();
    let attention = causal_attention(&normalized, &normalized, &normalized).unwrap();
    input
        .iter()
        .zip(&attention)
        .map(|(&original, &context)| {
            let residual = [original[0] + context[0], original[1] + context[1]];
            let norm = layer_norm(residual);
            // Упрощённый FFN: два ReLU-канала и фиксированная выходная проекция.
            [
                residual[0] + 0.2 * norm[0].max(0.0),
                residual[1] + 0.2 * norm[1].max(0.0),
            ]
        })
        .collect()
}
fn main() {
    let states = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let output = block(&states);
    assert_eq!(output.len(), states.len());
    println!("после decoder block: {output:?}");
}
