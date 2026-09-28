// Урок 21.4. Ограничение градиента.
//
// Значения внутри интервала [-limit, limit] не меняются. Выходящие за границу
// заменяются ближайшей границей с сохранением знака.

fn main() {
    let limit = 1.0;
    assert!(limit > 0.0);
    for (description, gradient, expected) in [
        ("слишком большой положительный", 12.0, 1.0),
        ("положительный внутри интервала", 0.5, 0.5),
        ("нулевой", 0.0, 0.0),
        ("отрицательный внутри интервала", -0.5, -0.5),
        ("слишком большой отрицательный", -12.0, -1.0),
    ] {
        let clipped = if gradient > limit {
            limit
        } else if gradient < -limit {
            -limit
        } else {
            gradient
        };
        assert_eq!(clipped, expected);
        println!("{description}: {gradient} → {clipped}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (-30..=30)
        .map(|i| {
            let g = i as f64 / 10.0;
            (g, g.clamp(-1.0, 1.0))
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ограничение градиента",
        "градиент до",
        "градиент после",
        &[lesson_visualization::Series {
            name: "порог ±1",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
