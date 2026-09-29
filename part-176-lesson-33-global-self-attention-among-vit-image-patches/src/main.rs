// Урок 33.2. Глобальное внимание между патчами изображения ViT.
// Зачем здесь эта тема: Локальные патчи должны обмениваться сведениями о далёких областях
//   изображения.
// Почему код устроен так: Считаем веса первого патча ко всем патчам и проверяем, что даже последний
//   доступен.
// Представь: Первый патч может получить вес внимания и от последнего, даже если они далеко друг от
//   друга на картинке.
// В классификации изображения патчи могут видеть друг друга без причинной маски.

// Нормируем оценки всех патчей в вероятностные веса.
// Оценку модели до преобразования в вероятность называют logit.
fn softmax_probabilities_from_raw_model_scores(raw_model_scores: &[f64]) -> Vec<f64> {
    let maximum: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    lesson_trace::trace_step!(maximum);
    let exponential_values: Vec<f64> = raw_model_scores
        .iter()
        .map(|&patch_value| (patch_value - maximum).exp())
        .collect();
    lesson_trace::trace_step!(exponential_values);
    let sum: f64 = exponential_values.iter().sum();
    lesson_trace::trace_step!(sum);
    exponential_values
        .into_iter()
        .map(|patch_value| patch_value / sum)
        .collect()
}
fn main() {
    lesson_trace::enable();
    let image: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    lesson_trace::trace_step!(image);
    let patches: Vec<Vec<f64>> =
        part_175_lesson_33_split_square_image_into_nonoverlapping_vit_patches::extract_nonoverlapping_square_patches_from_square_image(&image, 1).unwrap();
    lesson_trace::trace_step!(patches);
    // Упрощённая проекция одномерного патча в двухмерный токен.
    // Представление патча изображения для трансформера называют visual token.
    let image_patch_representations: Vec<[f64; 2]> = patches
        .iter()
        .map(|patch| [patch[0], 1.0 - patch[0]])
        .collect();
    lesson_trace::trace_step!(image_patch_representations);
    let first: [f64; 2] = image_patch_representations[0];
    lesson_trace::trace_step!(first);
    let raw_model_scores: Vec<f64> = image_patch_representations
        .iter()
        .map(|key| first[0] * key[0] + first[1] * key[1])
        .collect();
    lesson_trace::trace_step!(raw_model_scores);
    let weights: Vec<f64> = softmax_probabilities_from_raw_model_scores(&raw_model_scores);
    lesson_trace::trace_step!(weights);
    assert_eq!(weights.len(), 4);
    assert!(weights[3] > 0.0); // Последний патч виден первому.
    println!("веса внимания первого патча ко всем патчам: {weights:?}");
    lesson_trace::disable();
    visualize_global_self_attention_among_vit_image_patches(&weights);
}

fn visualize_global_self_attention_among_vit_image_patches(weights: &[f64]) {
    let labels: [&str; 4] = ["patch 0", "patch 1", "patch 2", "patch 3"];
    let values: Vec<(&str, f64)> = labels
        .iter()
        .zip(weights)
        .map(|(&label, &weight)| (label, weight))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "vit-attention",
        "Внимание первого патча",
        "вес",
        &values,
    )
    .expect("график");
    println!("график: {}", path.display());
}
