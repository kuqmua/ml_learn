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
use l179_33_create_image_block_sequence_by_splitting_image_into_nonoverlapping_squares::extract_nonoverlapping_square_patches_from_square_image;

fn calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum<
    const N: usize,
>(
    raw_model_scores: &[f64; N],
) -> [f64; N] {
    let maximum: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: [f64; N] =
        std::array::from_fn(|index| (raw_model_scores[index] - maximum).exp());
    let sum: f64 = exponential_values.iter().sum();
    exponential_values.map(|patch_value| patch_value / sum)
}
fn main() {
    let image: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
    let patches: Vec<Vec<f64>> =
        extract_nonoverlapping_square_patches_from_square_image(&image, 1).unwrap();
    let image_patch_representations: [[f64; 2]; 4] = patches
        .iter()
        .map(|patch| [patch[0], 1.0 - patch[0]])
        .collect::<Vec<_>>()
        .try_into()
        .expect("из изображения 2×2 получаются четыре патча 1×1");
    let first: [f64; 2] = image_patch_representations[0];
    let raw_model_scores: [f64; 4] = std::array::from_fn(|index| {
        let key = image_patch_representations[index];
        first[0] * key[0] + first[1] * key[1]
    });
    let weights: [f64; 4] =
        calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &raw_model_scores,
        );
    assert!(weights[3] > 0.0);

    plot_weights_assigned_from_first_image_block_to_all_blocks(&weights);
}

fn plot_weights_assigned_from_first_image_block_to_all_blocks(weights: &[f64; 4]) {
    let labels: [&str; 4] = ["patch 0", "patch 1", "patch 2", "patch 3"];
    let values: [(&str, f64); 4] = std::array::from_fn(|index| (labels[index], weights[index]));
    let _path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "vit-attention",
        "Внимание первого патча",
        "вес",
        &values,
    )
    .expect("график");
}
