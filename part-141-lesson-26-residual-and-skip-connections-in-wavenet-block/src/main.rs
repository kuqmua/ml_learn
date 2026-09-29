// Урок 26.4. Остаточные и сквозные связи в блоке WaveNet.
// Residual переносит состояние через слои, skip собирает вклады для выхода.

fn apply_wavenet_block_with_residual_and_skip_outputs(input: f64, transform: f64) -> (f64, f64) {
    let activation: f64 = (input * transform).tanh();
    // Добавление входа блока к его преобразованному выходу называют residual connection.
    let input_plus_transformed_value: f64 = input + activation;
    let skip: f64 = activation;
    (input_plus_transformed_value, skip)
}
fn main() {
    let mut state: f64 = 0.5;
    let mut skip_sum: f64 = 0.0;
    for transform in [0.2, -0.4, 0.8] {
        let (next, skip): (f64, f64) =
            apply_wavenet_block_with_residual_and_skip_outputs(state, transform);
        state = next;
        skip_sum += skip;
    }
    assert!((state - (0.5 + skip_sum)).abs() < 1e-12);
    println!("residual state={state:.4}; skip sum={skip_sum:.4}");
}
