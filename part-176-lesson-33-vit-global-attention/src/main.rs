// Урок 33.2. Глобальное внимание ViT.
// В классификации изображения патчи могут видеть друг друга без причинной маски.

use part_175_lesson_33_vit_patches::patches;

// Нормируем оценки всех патчей в вероятностные веса.
fn softmax(logits: &[f64]) -> Vec<f64> {
    let maximum = logits.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: Vec<f64> = logits
        .iter()
        .map(|&patch_value| (patch_value - maximum).exp())
        .collect();
    let sum: f64 = exponential_values.iter().sum();
    exponential_values
        .into_iter()
        .map(|patch_value| patch_value / sum)
        .collect()
}
fn main() {
    let image = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let patches = patches(&image, 1).unwrap();
    // Упрощённая проекция одномерного патча в двухмерный токен.
    let tokens: Vec<[f64; 2]> = patches
        .iter()
        .map(|patch| [patch[0], 1.0 - patch[0]])
        .collect();
    let first = tokens[0];
    let logits: Vec<f64> = tokens
        .iter()
        .map(|key| first[0] * key[0] + first[1] * key[1])
        .collect();
    let weights = softmax(&logits);
    assert_eq!(weights.len(), 4);
    assert!(weights[3] > 0.0); // Последний патч виден первому.
    println!("веса внимания первого патча ко всем патчам: {weights:?}");
    visualize(&weights);
}

fn visualize(weights: &[f64]) {
    let labels = ["patch 0", "patch 1", "patch 2", "patch 3"];
    let values: Vec<_> = labels
        .iter()
        .zip(weights)
        .map(|(&label, &weight)| (label, weight))
        .collect();
    let path = lesson_visualization::bars(
        env!("CARGO_MANIFEST_DIR"),
        "vit-attention",
        "Внимание первого патча",
        "вес",
        &values,
    )
    .expect("график");
    println!("график: {}", path.display());
}
