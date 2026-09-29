// Урок 16.1. Обучение следующего дерева на остатках текущей модели.
// Зачем здесь эта тема: В отличие от независимых моделей ансамбля, бустинг исправляет оставшиеся
//   ошибки последовательно.
// Почему код устроен так: Следующее дерево получает остатки после текущего прогноза, поэтому его
//   цель меняется на каждом шаге.
// Представь: Если модель предсказала 8 вместо 10, следующая модель учится исправлять остаток +2.
// Каждое следующее дерево исправляет остатки текущей модели.

fn main() {
    lesson_trace::enable();
    // Простая регрессия без случайности позволяет проверить каждый шаг вручную.
    let targets: [f64; 4] = [0.0, 0.0, 2.0, 2.0];
    lesson_trace::trace_step!(targets);
    let mut predictions: [f64; 4] = [1.0; 4];
    lesson_trace::trace_step!(predictions);
    let mut history: Vec<f64> = vec![1.0];
    lesson_trace::trace_step!(history);
    // Два раунда позволяют увидеть, как второй маленький «деревянный» шаг исправляет первый.
    for round in 0..2 {
        lesson_trace::trace_step!(round);
        // Для квадратичной ошибки отрицательный градиент равен y - prediction.
        // Разность целевого значения и прогноза называют residual (остатком).
        let target_minus_prediction_values: [f64; 4] =
            std::array::from_fn::<_, 4, _>(|step_index| {
                targets[step_index] - predictions[step_index]
            });
        lesson_trace::trace_step!(target_minus_prediction_values);
        let left: f64 =
            (target_minus_prediction_values[0] + target_minus_prediction_values[1]) / 2.0;
        lesson_trace::trace_step!(left);
        let right: f64 =
            (target_minus_prediction_values[2] + target_minus_prediction_values[3]) / 2.0;
        lesson_trace::trace_step!(right);
        // Берём половину предсказанного остатка (shrinkage = 0.5), чтобы исправлять ошибку постепенно.
        for (index, value) in predictions.iter_mut().enumerate() {
            lesson_trace::trace_step!(index);
            lesson_trace::trace_step!(value);
            *value += 0.5 * if index < 2 { left } else { right };
            lesson_trace::trace_step!(value);
        }
        let mean_squared_error_value: f64 = targets
            .iter()
            .zip(predictions)
            .map(|(target, prediction)| (target - prediction).powi(2))
            .sum::<f64>()
            / 4.0;
        lesson_trace::trace_step!(mean_squared_error_value);
        history.push(mean_squared_error_value);
        println!(
            "итерация {}: prediction={predictions:?}, MSE={mean_squared_error_value}",
            round + 1
        );
    }
    assert_eq!(predictions, [0.25, 0.25, 1.75, 1.75]);
    lesson_trace::disable();
    visualize_fit_next_boosting_tree_to_current_residuals(&history);
}

fn visualize_fit_next_boosting_tree_to_current_residuals(losses: &[f64]) {
    let points: Vec<(f64, f64)> = losses
        .iter()
        .enumerate()
        .map(|(item_index, &loss)| (item_index as f64, loss))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "boosting-error",
        "Ошибка после каждого дерева",
        "итерация",
        "MSE",
        &[lesson_visualization::Series {
            name: "MSE",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
