// Урок 35.5. Отрицательный логарифм вероятности правильной следующей части текста.
// Связь с принятой терминологией: Кросс энтропия прогноза следующего токена.
// Зачем здесь эта тема: Выход decoder — сырые логиты; для обучения нужен штраф за неверный
//   следующий токен.
// Почему код устроен так: Нормируем логиты и берём отрицательный логарифм вероятности правильного
//   id.
// Представь: Если правильное следующее слово имеет низкую вероятность, кросс энтропия получается
//   большой.
// Логиты позиции t оцениваются по целевому токену позиции t+1.

// Оценку модели до преобразования в вероятность называют logit.
/// Перекрёстная энтропия: получаем вероятности через softmax, выбираем правильный токен и берём −ln(p).
fn calculate_next_token_loss_as_negative_log_of_target_probability_from_exponentiated_scores(
    raw_model_scores: &[f64],
    target: usize,
) -> f64 {
    -part_182_lesson_35_sum_current_and_past_values_with_query_key_match_weights::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
        raw_model_scores,
    )[target]
        .ln()
}
fn main() {
    lesson_trace::enable();
    // BOS, A, B, EOS: на последней позиции нет следующей цели.
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifiers: [usize; 4] = [0, 1, 2, 3];
    lesson_trace::trace_step!(text_unit_identifiers);
    let raw_model_scores: [[f64; 4]; 3] = [
        [0.2, 2.0, 0.1, 0.0],
        [0.1, 0.2, 2.0, 0.0],
        [0.0, 0.1, 0.2, 2.0],
    ];
    lesson_trace::trace_step!(raw_model_scores);
    let losses: Vec<f64> = (0..text_unit_identifiers.len() - 1)
        .map(|plot_step_index| {
            calculate_next_token_loss_as_negative_log_of_target_probability_from_exponentiated_scores(
                &raw_model_scores[plot_step_index],
                text_unit_identifiers[plot_step_index + 1],
            )
        })
        .collect();
    lesson_trace::trace_step!(losses);
    let average: f64 = losses.iter().sum::<f64>() / losses.len() as f64;
    lesson_trace::trace_step!(average);
    let wrong: f64 =
        calculate_next_token_loss_as_negative_log_of_target_probability_from_exponentiated_scores(
            &[2.0, 0.2, 0.1, 0.0],
            text_unit_identifiers[1],
        );
    lesson_trace::trace_step!(wrong);
    assert!(average < wrong);
    println!("loss по позициям: {losses:?}; средний loss={average:.3}");
    lesson_trace::disable();
    plot_next_token_loss_as_negative_log_correct_text_unit_probability_by_position(&losses);
}
fn plot_next_token_loss_as_negative_log_correct_text_unit_probability_by_position(losses: &[f64]) {
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
