// Урок 25.3. CLS-токен в Vision Transformer.
// Специальный токен собирает информацию от патчей для классификации изображения.

fn softmax(values: &[f64]) -> Vec<f64> {
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let exp: Vec<_> = values.iter().map(|&x| (x - max).exp()).collect();
    let sum: f64 = exp.iter().sum();
    exp.iter().map(|x| x / sum).collect()
}
fn main() {
    // Первый токен обозначает CLS; остальные представляют патчи.
    let tokens = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let logits: Vec<f64> = tokens
        .iter()
        .map(|token| tokens[0][0] * token[0] + tokens[0][1] * token[1])
        .collect();
    let weights = softmax(&logits);
    let pooled = tokens
        .iter()
        .zip(&weights)
        .fold([0.0; 2], |mut sum, (x, &weight)| {
            sum[0] += weight * x[0];
            sum[1] += weight * x[1];
            sum
        });
    let class = u8::from(pooled[0] > pooled[1]);
    assert_eq!(weights.len(), 3);
    println!("CLS context={pooled:?}; class={class}");
    visualize(&weights);
}
fn visualize(weights: &[f64]) {
    let values = [
        ("CLS", weights[0]),
        ("patch 1", weights[1]),
        ("patch 2", weights[2]),
    ];
    let path = lesson_visualization::bars(
        env!("CARGO_MANIFEST_DIR"),
        "cls-weights",
        "Что читает CLS",
        "вес",
        &values,
    )
    .expect("график");
    println!("график: {}", path.display());
}
