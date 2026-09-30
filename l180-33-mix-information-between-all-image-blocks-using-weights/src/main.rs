// Урок 33.2. Обмен информацией между всеми участками изображения через взвешенное сложение.
// Связь с принятой терминологией: Глобальное внимание между патчами изображения ViT.
// Зачем здесь эта тема: Локальные патчи должны обмениваться сведениями о далёких областях
//   изображения.
// Почему код устроен так: Считаем веса первого патча ко всем патчам и проверяем, что даже последний
//   доступен.
// Представь: Первый патч может получить вес внимания и от последнего, даже если они далеко друг от
//   друга на картинке.
// В классификации изображения патчи могут видеть друг друга без причинной маски.

// Нормируем оценки всех патчей в вероятностные веса.
// Оценку модели до преобразования в вероятность называют logit.
/// Softmax: вычитаем максимальную оценку, вычисляем экспоненты и делим каждую на их сумму.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
    raw_model_scores: &[f64],
) -> Vec<f64> {
    let maximum: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    trace_step!(maximum);
    let exponential_values: Vec<f64> = raw_model_scores
        .iter()
        .map(|&patch_value| (patch_value - maximum).exp())
        .collect();
    trace_step!(exponential_values);
    let sum: f64 = exponential_values.iter().sum();
    trace_step!(sum);
    exponential_values
        .into_iter()
        .map(|patch_value| patch_value / sum)
        .collect()
}
fn main() {
    enable();
    let image: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    trace_step!(image);
    let patches: Vec<Vec<f64>> =
        l179_33_create_image_block_sequence_by_splitting_image_into_nonoverlapping_squares::extract_nonoverlapping_square_patches_from_square_image(&image, 1).unwrap();
    trace_step!(patches);
    trace_note!("Упрощённая проекция одномерного патча в двухмерный токен.");
    trace_note!("Представление патча изображения для трансформера называют visual token.");
    let image_patch_representations: Vec<[f64; 2]> = patches
        .iter()
        .map(|patch| [patch[0], 1.0 - patch[0]])
        .collect();
    trace_step!(image_patch_representations);
    let first: [f64; 2] = image_patch_representations[0];
    trace_step!(first);
    let raw_model_scores: Vec<f64> = image_patch_representations
        .iter()
        .map(|key| first[0] * key[0] + first[1] * key[1])
        .collect();
    trace_step!(raw_model_scores);
    let weights: Vec<f64> =
        calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &raw_model_scores,
        );
    trace_step!(weights);
    assert_eq!(weights.len(), 4);
    assert!(weights[3] > 0.0);
    trace_note!("Последний патч виден первому.");

    println!("веса внимания первого патча ко всем патчам: {weights:?}");
    disable();
    plot_weights_assigned_from_first_image_block_to_all_blocks(&weights);
}

fn plot_weights_assigned_from_first_image_block_to_all_blocks(weights: &[f64]) {
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
