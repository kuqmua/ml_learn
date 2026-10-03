// Урок 16.1. Обучение следующего дерева на ошибках текущих прогнозов.
// Зачем здесь эта тема: В отличие от независимых моделей ансамбля, бустинг исправляет оставшиеся
//   ошибки последовательно.
// Почему код устроен так: Следующее дерево получает остатки после текущего прогноза, поэтому его
//   цель меняется на каждом шаге.
// Представь: Если модель предсказала 8 вместо 10, следующая модель учится исправлять остаток +2.
// Каждое следующее дерево исправляет остатки текущей модели.

fn main() {
    let targets: [f64; 4] = [0.0, 0.0, 2.0, 2.0];
    let mut predictions: [f64; 4] = [1.0; 4];
    let mut history: [f64; 3] = [1.0; 3];
    for round in 0..2 {
        let target_minus_prediction_values: [f64; 4] =
            std::array::from_fn::<_, 4, _>(|step_index| {
                targets[step_index] - predictions[step_index]
            });
        let average_error_for_first_group: f64 =
            (target_minus_prediction_values[0] + target_minus_prediction_values[1]) / 2.0;
        let average_error_for_second_group: f64 =
            (target_minus_prediction_values[2] + target_minus_prediction_values[3]) / 2.0;
        for (index, value) in predictions.iter_mut().enumerate() {
            *value += 0.5
                * if index < 2 {
                    average_error_for_first_group
                } else {
                    average_error_for_second_group
                };
        }
        let mean_squared_error_where_0_means_exact_predictions_and_larger_means_worse: f64 = targets
            .iter()
            .zip(predictions)
            .map(|(target, prediction)| (target - prediction).powi(2))
            .sum::<f64>()
            / 4.0;
        history[round + 1] =
            mean_squared_error_where_0_means_exact_predictions_and_larger_means_worse;
    }
    assert_eq!(predictions, [0.25, 0.25, 1.75, 1.75]);
    plot_average_squared_error_after_each_added_tree(&history);
}

fn plot_average_squared_error_after_each_added_tree(losses: &[f64; 3]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "boosting-error",
        "Ошибка после каждого дерева",
        "итерация",
        "MSE",
        &[lesson_visualization::Series {
            name: "MSE",
            points: &losses
                .iter()
                .enumerate()
                .map(|(item_index, &loss)| (item_index as f64, loss))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
