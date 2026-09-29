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
    values: &[f64],
) -> Vec<f64> {
    let maximum_value: f64 = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    lesson_trace::trace_step!(maximum_value);
    let exponential_values: Vec<f64> = values
        .iter()
        .map(|&patch_value| (patch_value - maximum_value).exp())
        .collect();
    lesson_trace::trace_step!(exponential_values);
    let sum: f64 = exponential_values.iter().sum();
    lesson_trace::trace_step!(sum);
    exponential_values
        .iter()
        .map(|patch_value| patch_value / sum)
        .collect()
}
fn main() {
    lesson_trace::enable();
    // Первый токен обозначает CLS; остальные представляют патчи.
    // Патчи и элемент классификации в ViT называют visual tokens.
    let image_input_representations: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    lesson_trace::trace_step!(image_input_representations);
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_scores: Vec<f64> = image_input_representations
        .iter()
        .map(|image_input_representation| {
            image_input_representations[0][0] * image_input_representation[0]
                + image_input_representations[0][1] * image_input_representation[1]
        })
        .collect();
    lesson_trace::trace_step!(raw_model_scores);
    let weights: Vec<f64> =
        calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &raw_model_scores,
        );
    lesson_trace::trace_step!(weights);
    // Итоговое представление элемента классификации получают через class token pooling.
    let image_classification_summary: [f64; 2] = image_input_representations
        .iter()
        .zip(&weights)
        .fold([0.0; 2], |mut sum, (patch_value, &weight)| {
            sum[0] += weight * patch_value[0];
            lesson_trace::trace_step!(sum);
            sum[1] += weight * patch_value[1];
            lesson_trace::trace_step!(sum);
            sum
        });
    lesson_trace::trace_step!(image_classification_summary);
    let class: u8 = u8::from(image_classification_summary[0] > image_classification_summary[1]);
    lesson_trace::trace_step!(class);
    assert_eq!(weights.len(), 3);
    println!("CLS context={image_classification_summary:?}; class={class}");
    lesson_trace::disable();
    plot_weights_used_to_combine_image_blocks_into_summary(&weights);
}
fn plot_weights_used_to_combine_image_blocks_into_summary(weights: &[f64]) {
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
