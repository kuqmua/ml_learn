// Урок 35.5. Потери при предсказании следующего токена.
// Логиты позиции t оцениваются по целевому токену позиции t+1.

// Оценку модели до преобразования в вероятность называют logit.
fn cross_entropy(raw_model_scores: &[f64], target: usize) -> f64 {
    -part_182_lesson_35_causal_self_attention::softmax(raw_model_scores)[target].ln()
}
fn main() {
    // BOS, A, B, EOS: на последней позиции нет следующей цели.
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifiers = [0, 1, 2, 3];
    let raw_model_scores = [
        [0.2, 2.0, 0.1, 0.0],
        [0.1, 0.2, 2.0, 0.0],
        [0.0, 0.1, 0.2, 2.0],
    ];
    let losses: Vec<f64> = (0..text_unit_identifiers.len() - 1)
        .map(|plot_step_index| {
            cross_entropy(
                &raw_model_scores[plot_step_index],
                text_unit_identifiers[plot_step_index + 1],
            )
        })
        .collect();
    let average = losses.iter().sum::<f64>() / losses.len() as f64;
    let wrong = cross_entropy(&[2.0, 0.2, 0.1, 0.0], text_unit_identifiers[1]);
    assert!(average < wrong);
    println!("loss по позициям: {losses:?}; средний loss={average:.3}");
    visualize(&losses);
}
fn visualize(losses: &[f64]) {
    let points: Vec<_> = losses
        .iter()
        .enumerate()
        .map(|(item_index, &loss)| (item_index as f64, loss))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "next-token-loss",
        "Потери по позициям",
        "позиция",
        "cross entropy",
        &[lesson_visualization::Series {
            name: "loss",
            points: &points,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", path.display());
}
