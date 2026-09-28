// Урок 09.2. Средняя абсолютная ошибка.
//
// Ошибки разных знаков не сокращаются; удвоение промаха удваивает вклад в MAE.
// Общая библиотека проверяет, что у каждого прогноза есть правильный ответ.

fn main() {
    let targets = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 4] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка выше ответа", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка ниже ответа", &[2.0, 3.0, 6.0], 1.0 / 3.0),
        ("ошибка вдвое больше", &[2.0, 6.0, 6.0], 2.0 / 3.0),
    ];
    for (description, predictions, expected) in cases {
        let mae = part_048_lesson_09_mae::mean_absolute_error(&targets, predictions)
            .expect("у каждого прогноза есть правильный ответ");
        assert!((mae - expected).abs() < 1e-10);
        println!("{description}: {predictions:?} → MAE {mae:.3}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (-30..=30)
        .map(|i| {
            let d = i as f64 / 10.0;
            (
                d,
                part_048_lesson_09_mae::mean_absolute_error(
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
        "Средняя абсолютная ошибка",
        "смещение прогноза",
        "MAE",
        &[lesson_visualization::Series {
            name: "цели [2,4,6]",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
