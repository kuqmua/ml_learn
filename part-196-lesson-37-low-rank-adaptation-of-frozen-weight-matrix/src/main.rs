// Урок 37.1. Низкоранговая адаптация замороженной матрицы весов.
// Замороженную матрицу дополняют произведением маленьких обучаемых матриц.

fn main() {
    let frozen = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    // Rank 1: A имеет форму 4x1, B — 1x4.
    let adapter_output_weights = [0.1, 0.2, 0.3, 0.4];
    let adapter_input_weights = [1.0, 0.0, -1.0, 0.0];
    let input = [1.0, 2.0, 3.0, 4.0];
    let projected_input: f64 = adapter_input_weights
        .iter()
        .zip(input)
        .map(|(adapter_component, input_component)| adapter_component * input_component)
        .sum();
    let output: [f64; 4] = std::array::from_fn(|row| {
        frozen[row]
            .iter()
            .zip(input)
            .map(|(weight_value, input_component)| weight_value * input_component)
            .sum::<f64>()
            + adapter_output_weights[row] * projected_input
    });
    assert_eq!(output[0], 0.8);
    println!("выход с LoRA: {output:?}; trainable=8 вместо 16 параметров");
}
