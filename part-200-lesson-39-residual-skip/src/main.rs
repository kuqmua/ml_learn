// Урок 39.4. Residual и skip связи.
// Residual переносит состояние через слои, skip собирает вклады для выхода.

fn block(input: f64, transform: f64) -> (f64, f64) {
    let activation = (input * transform).tanh();
    let residual = input + activation;
    let skip = activation;
    (residual, skip)
}
fn main() {
    let mut state = 0.5;
    let mut skip_sum = 0.0;
    for transform in [0.2, -0.4, 0.8] {
        let (next, skip) = block(state, transform);
        state = next;
        skip_sum += skip;
    }
    assert!((state - (0.5 + skip_sum)).abs() < 1e-12);
    println!("residual state={state:.4}; skip sum={skip_sum:.4}");
}
