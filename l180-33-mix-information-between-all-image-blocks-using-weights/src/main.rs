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
// Оценка модели до преобразования в вероятность — обычное число, которое затем переводят в диапазон от 0 до 1.
/// Softmax: вычитаем максимальную оценку, вычисляем экспоненты и делим каждую на их сумму.
use l179_33_create_image_block_sequence_by_splitting_image_into_nonoverlapping_squares::extract_nonoverlapping_square_patches_from_square_image;

fn calculate_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
    raw_model_scores: &[f64; 4],
) -> [f64; 4] {
    let maximum: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: [f64; 4] =
        std::array::from_fn(|index| (raw_model_scores[index] - maximum).exp());
    let sum_of_exponential_values: f64 = exponential_values.iter().sum();
    exponential_values.map(|patch_value| patch_value / sum_of_exponential_values)
}
fn main() {
    let image: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];

    let image_patch_representations: [[f64; 2]; 4] =
        extract_nonoverlapping_square_patches_from_square_image(&image, 1)
            .unwrap()
            .iter()
            .map(|patch| [patch[0], 1.0 - patch[0]])
            .collect::<Vec<_>>()
            .try_into()
            .expect("ожидалось четыре патча 1×1 из изображения 2×2");
    let first_patch_representation: [f64; 2] = image_patch_representations[0];
    let weights: [f64; 4] =
        calculate_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
            &std::array::from_fn::<f64, 4, _>(|index| {
                let key = image_patch_representations[index];
                first_patch_representation[0] * key[0] + first_patch_representation[1] * key[1]
            }),
        );
    assert!(weights[3] > 0.0);

    plot_weights_assigned_from_first_image_block_to_all_blocks(&weights);
}

fn plot_weights_assigned_from_first_image_block_to_all_blocks(weights: &[f64; 4]) {
    let labels: [&str; 4] = ["patch 0", "patch 1", "patch 2", "patch 3"];
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "vit-attention",
        "Внимание первого патча",
        "вес",
        &std::array::from_fn::<(&str, f64), 4, _>(|index| (labels[index], weights[index])),
    )
    .expect("не удалось сохранить график");
}
