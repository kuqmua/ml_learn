// Урок 16.1. Бустинг по остаткам.
// Каждое следующее дерево исправляет остатки текущей модели.

fn main() {
    // Простая регрессия без случайности позволяет проверить каждый шаг вручную.
    let targets: [f64; 4] = [0.0, 0.0, 2.0, 2.0];
    let mut predictions = [1.0; 4];
    let mut history = vec![1.0];
    for round in 0..2 {
        // Для квадратичной ошибки отрицательный градиент равен y - prediction.
        let residuals = std::array::from_fn::<_, 4, _>(|step_index| {
            targets[step_index] - predictions[step_index]
        });
        let left = (residuals[0] + residuals[1]) / 2.0;
        let right = (residuals[2] + residuals[3]) / 2.0;
        for (index, value) in predictions.iter_mut().enumerate() {
            *value += 0.5 * if index < 2 { left } else { right };
        }
        let mean_squared_error_value: f64 = targets
            .iter()
            .zip(predictions)
            .map(|(target, pred)| (target - pred).powi(2))
            .sum::<f64>()
            / 4.0;
        history.push(mean_squared_error_value);
        println!(
            "итерация {}: prediction={predictions:?}, MSE={mean_squared_error_value}",
            round + 1
        );
    }
    assert_eq!(predictions, [0.25, 0.25, 1.75, 1.75]);
    visualize(&history);
}

fn visualize(losses: &[f64]) {
    let points: Vec<_> = losses
        .iter()
        .enumerate()
        .map(|(item_index, &loss)| (item_index as f64, loss))
        .collect();
    let path = lesson_visualization::line_chart(
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
