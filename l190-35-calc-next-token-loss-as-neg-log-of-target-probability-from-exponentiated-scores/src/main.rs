// Урок 35.6. Ошибка прогноза следующей части текста: отрицательный логарифм правильной вероятности.
// Зачем здесь эта тема: Выход decoder — сырые логиты; для обучения нужен штраф за неверный
//   следующий токен.
// Почему код устроен так: Нормируем логиты и берём отрицательный логарифм вероятности правильного
//   id.
// Представь: Если правильное следующее слово имеет низкую вероятность, кросс энтропия получается
//   большой.
// Логиты позиции t оцениваются по целевому токену позиции t+1.

// Оценка модели до преобразования в вероятность — обычное число, которое затем переводят в диапазон от 0 до 1.
/// Перекрёстная энтропия: получаем вероятности через softmax, выбираем правильный токен и берём −ln(p).
use l186_35_calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum;

fn calc_next_token_loss_as_neg_log_of_target_probability_from_exponentiated_scores(
    raw_model_scores: &[f64],
    target: usize,
) -> f64 {
    -calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
        raw_model_scores,
    )[target]
        .ln()
}
fn main() {
    let text_unit_identifiers: [usize; 4] = [0, 1, 2, 3];
    let raw_model_scores: [[f64; 4]; 3] = [
        [0.2, 2.0, 0.1, 0.0],
        [0.1, 0.2, 2.0, 0.0],
        [0.0, 0.1, 0.2, 2.0],
    ];
    let neg_log_correct_token_probabilities: [f64; 3] = std::array::from_fn(|plot_step_index| {
        calc_next_token_loss_as_neg_log_of_target_probability_from_exponentiated_scores(
            &raw_model_scores[plot_step_index],
            text_unit_identifiers[plot_step_index + 1],
        )
    });
    let mean_neg_log_correct_token_probability: f64 =
        neg_log_correct_token_probabilities.iter().sum::<f64>()
            / neg_log_correct_token_probabilities.len() as f64;

    assert!(
        mean_neg_log_correct_token_probability
            < calc_next_token_loss_as_neg_log_of_target_probability_from_exponentiated_scores(
                &[2.0, 0.2, 0.1, 0.0],
                text_unit_identifiers[1],
            )
    );

    plot_next_token_loss_as_neg_log_correct_text_unit_probability_by_position(
        &neg_log_correct_token_probabilities,
    );
}
fn plot_next_token_loss_as_neg_log_correct_text_unit_probability_by_position(
    neg_log_correct_token_probabilities: &[f64; 3],
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "next-token-loss",
        "Потери по позициям",
        "позиция",
        "cross entropy",
        &[lesson_visualization::Series {
            name: "loss",
            points: &neg_log_correct_token_probabilities
                .iter()
                .enumerate()
                .map(|(item_index, &loss)| (item_index as f64, loss))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
