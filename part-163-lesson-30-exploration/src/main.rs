// Урок 30.5. Исследование и использование.
//
// Случайное число ниже epsilon ведёт к исследованию, иначе выбираем лучшее известное действие.
// На границе random=epsilon выбираем использование.

fn main() {
    let exploration_probability = 0.1;
    let best_known_action = "вправо";
    for (description, random_fraction, expected_action) in [
        ("исследование", 0.05, "влево"),
        ("граница", 0.1, "вправо"),
        ("использование", 0.8, "вправо"),
    ] {
        assert!((0.0..1.0).contains(&random_fraction));
        let action = if random_fraction < exploration_probability {
            "влево"
        } else {
            best_known_action
        };
        assert_eq!(action, expected_action);
        println!("{description}: случайное число={random_fraction}, действие={action}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (0..=100)
        .map(|i| {
            let u = i as f64 / 100.0;
            (u, if u < 0.2 { 1.0 } else { 0.0 })
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Исследование и использование",
        "случайное число",
        "выбор",
        &[lesson_visualization::Series {
            name: "порог ε=0.2: 1=исследование",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
