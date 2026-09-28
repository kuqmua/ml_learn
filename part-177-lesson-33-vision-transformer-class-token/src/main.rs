// Урок 33.3. CLS-токен в Vision Transformer.
// Специальный токен собирает информацию от патчей для классификации изображения.

fn softmax(values: &[f64]) -> Vec<f64> {
    let maximum_value = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: Vec<_> = values
        .iter()
        .map(|&patch_value| (patch_value - maximum_value).exp())
        .collect();
    let sum: f64 = exponential_values.iter().sum();
    exponential_values
        .iter()
        .map(|patch_value| patch_value / sum)
        .collect()
}
fn main() {
    // Первый токен обозначает CLS; остальные представляют патчи.
    // Патчи и элемент классификации в ViT называют visual tokens.
    let image_input_representations = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_scores: Vec<f64> = image_input_representations
        .iter()
        .map(|image_input_representation| {
            image_input_representations[0][0] * image_input_representation[0]
                + image_input_representations[0][1] * image_input_representation[1]
        })
        .collect();
    let weights = softmax(&raw_model_scores);
    // Итоговое представление элемента классификации получают через class token pooling.
    let image_classification_summary = image_input_representations.iter().zip(&weights).fold(
        [0.0; 2],
        |mut sum, (patch_value, &weight)| {
            sum[0] += weight * patch_value[0];
            sum[1] += weight * patch_value[1];
            sum
        },
    );
    let class = u8::from(image_classification_summary[0] > image_classification_summary[1]);
    assert_eq!(weights.len(), 3);
    println!("CLS context={image_classification_summary:?}; class={class}");
    visualize(&weights);
}
fn visualize(weights: &[f64]) {
    let values = [
        ("CLS", weights[0]),
        ("patch 1", weights[1]),
        ("patch 2", weights[2]),
    ];
    let path = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "cls-weights",
        "Что читает CLS",
        "вес",
        &values,
    )
    .expect("график");
    println!("график: {}", path.display());
}
