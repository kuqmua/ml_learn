// Урок 35.5. Кросс энтропия прогноза следующего токена.
// Логиты позиции t оцениваются по целевому токену позиции t+1.

// Оценку модели до преобразования в вероятность называют logit.
fn cross_entropy_loss_for_target_token_from_logits(raw_model_scores: &[f64], target: usize) -> f64 {
    -part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::softmax_probabilities_from_raw_model_scores(
        raw_model_scores,
    )[target]
        .ln()
}
fn main() {
    // BOS, A, B, EOS: на последней позиции нет следующей цели.
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifiers: [usize; 4] = [0, 1, 2, 3];
    let raw_model_scores: [[f64; 4]; 3] = [
        [0.2, 2.0, 0.1, 0.0],
        [0.1, 0.2, 2.0, 0.0],
        [0.0, 0.1, 0.2, 2.0],
    ];
    let losses: Vec<f64> = (0..text_unit_identifiers.len() - 1)
        .map(|plot_step_index| {
            cross_entropy_loss_for_target_token_from_logits(
                &raw_model_scores[plot_step_index],
                text_unit_identifiers[plot_step_index + 1],
            )
        })
        .collect();
    let average: f64 = losses.iter().sum::<f64>() / losses.len() as f64;
    let wrong: f64 = cross_entropy_loss_for_target_token_from_logits(
        &[2.0, 0.2, 0.1, 0.0],
        text_unit_identifiers[1],
    );
    assert!(average < wrong);
    println!("loss по позициям: {losses:?}; средний loss={average:.3}");
    visualize_cross_entropy_loss_for_next_token_prediction(&losses);
}
fn visualize_cross_entropy_loss_for_next_token_prediction(losses: &[f64]) {
    let points: Vec<(f64, f64)> = losses
        .iter()
        .enumerate()
        .map(|(item_index, &loss)| (item_index as f64, loss))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
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
