// Урок 09.1. Среднеквадратичная ошибка.
//
// При точном прогнозе MSE равна нулю. Ошибка вдвое больше даёт вклад вчетверо больше.
// Та же общая функция будет использоваться для оценки моделей в следующих уроках.

fn main() {
    let targets = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 3] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    for (description, predictions, expected) in cases {
        let mse = part_047_lesson_09_mse::mean_squared_error(&targets, predictions)
            .expect("у каждого прогноза есть правильный ответ");
        assert!((mse - expected).abs() < 1e-10);
        println!("{description}: {predictions:?} → MSE {mse:.3}");
    }
    let error = part_047_lesson_09_mse::mean_squared_error(&targets, &[2.0, 4.0])
        .expect_err("длины должны совпадать");
    println!("разная длина: {error}");
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (-30..=30)
        .map(|i| {
            let d = i as f64 / 10.0;
            (
                d,
                part_047_lesson_09_mse::mean_squared_error(
                    &[2.0, 4.0, 6.0],
                    &[2.0 + d, 4.0 + d, 6.0 + d],
                )
                .unwrap(),
            )
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Среднеквадратичная ошибка",
        "смещение прогноза",
        "MSE",
        &[lesson_visualization::Series {
            name: "цели [2,4,6]",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
