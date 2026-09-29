// Урок 33.3. Сбор сведений о патчах изображения ViT в токене класса.
// Специальный токен собирает информацию от патчей для классификации изображения.

fn softmax_probabilities_from_raw_model_scores(values: &[f64]) -> Vec<f64> {
    let maximum_value: f64 = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: Vec<f64> = values
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
    let image_input_representations: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_scores: Vec<f64> = image_input_representations
        .iter()
        .map(|image_input_representation| {
            image_input_representations[0][0] * image_input_representation[0]
                + image_input_representations[0][1] * image_input_representation[1]
        })
        .collect();
    let weights: Vec<f64> = softmax_probabilities_from_raw_model_scores(&raw_model_scores);
    // Итоговое представление элемента классификации получают через class token pooling.
    let image_classification_summary: [f64; 2] = image_input_representations
        .iter()
        .zip(&weights)
        .fold([0.0; 2], |mut sum, (patch_value, &weight)| {
            sum[0] += weight * patch_value[0];
            sum[1] += weight * patch_value[1];
            sum
        });
    let class: u8 = u8::from(image_classification_summary[0] > image_classification_summary[1]);
    assert_eq!(weights.len(), 3);
    println!("CLS context={image_classification_summary:?}; class={class}");
    visualize_aggregate_vit_image_patches_into_class_token(&weights);
}
fn visualize_aggregate_vit_image_patches_into_class_token(weights: &[f64]) {
    let values: [(&str, f64); 3] = [
        ("CLS", weights[0]),
        ("patch 1", weights[1]),
        ("patch 2", weights[2]),
    ];
    let path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "cls-weights",
        "Что читает CLS",
        "вес",
        &values,
    )
    .expect("график");
    println!("график: {}", path.display());
}
