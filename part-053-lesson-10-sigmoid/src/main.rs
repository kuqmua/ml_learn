// Урок 10.1. Сигмоидная функция.
//
// Отрицательный logit даёт вероятность ниже 0.5, нулевой — 0.5,
// положительный — выше 0.5. Значение всегда находится между 0 и 1.

fn main() {
    for (description, logit, expected_side) in [
        ("отрицательный", -2.0, -1),
        ("нулевой", 0.0, 0),
        ("положительный", 2.0, 1),
    ] {
        let mut term = 1.0;
        let mut exponential = 1.0;
        for index in 1..=30 {
            term *= -logit / index as f64;
            exponential += term;
        }
        let probability = 1.0 / (1.0 + exponential);
        assert!(probability > 0.0 && probability < 1.0);
        let side = if probability < 0.5 {
            -1
        } else if probability > 0.5 {
            1
        } else {
            0
        };
        assert_eq!(side, expected_side);
        println!("{description} logit {logit}: вероятность {probability:.4}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (-60..=60)
        .map(|i| {
            let x = i as f64 / 10.0;
            (x, 1.0 / (1.0 + (-x).exp()))
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сигмоида",
        "логит",
        "вероятность",
        &[lesson_visualization::Series {
            name: "σ(x)",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
