// Урок 33.3. Объединение информации об участках изображения в один итоговый вектор.
// Связь с принятой терминологией: Сбор сведений о патчах изображения ViT в токене класса.
// Зачем здесь эта тема: Для классификации нужен один итоговый вектор, а внимание выдаёт вектор для
//   каждого патча.
// Почему код устроен так: Добавляем токен класса, который собирает сведения с позиций патчей.
// Представь: После общения патчей отдельный токен класса собирает информацию для одного прогноза
//   изображения.
// Специальный токен собирает информацию от патчей для классификации изображения.

/// Softmax: вычитаем максимальную оценку, вычисляем экспоненты и делим каждую на их сумму.

fn calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
    values: &[f64; 3],
) -> [f64; 3] {
    let maximum_value: f64 = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: [f64; 3] =
        std::array::from_fn(|index| (values[index] - maximum_value).exp());
    let sum_of_exponential_values: f64 = exponential_values.iter().sum();
    exponential_values.map(|patch_value| patch_value / sum_of_exponential_values)
}
fn main() {
    let image_input_representations: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let weights: [f64; 3] =
        calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &std::array::from_fn::<f64, 3, _>(|index| {
                let image_input_representation = image_input_representations[index];
                image_input_representations[0][0] * image_input_representation[0]
                    + image_input_representations[0][1] * image_input_representation[1]
            }),
        );
    let image_classification_summary: [f64; 2] =
        image_input_representations.iter().zip(&weights).fold(
            [0.0; 2],
            |mut weighted_coordinate_sums, (patch_value, &weight)| {
                weighted_coordinate_sums[0] += weight * patch_value[0];
                weighted_coordinate_sums[1] += weight * patch_value[1];
                weighted_coordinate_sums
            },
        );
    let _class: u8 = u8::from(image_classification_summary[0] > image_classification_summary[1]);

    plot_weights_used_to_combine_image_blocks_into_summary(&weights);
}
fn plot_weights_used_to_combine_image_blocks_into_summary(weights: &[f64; 3]) {
    let values: [(&str, f64); 3] = [
        ("CLS", weights[0]),
        ("patch 1", weights[1]),
        ("patch 2", weights[2]),
    ];
    let _path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "cls-weights",
        "Что читает CLS",
        "вес",
        &values,
    )
    .expect("не удалось сохранить график");
}
