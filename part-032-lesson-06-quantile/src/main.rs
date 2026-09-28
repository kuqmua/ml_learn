// Урок 06.4. Квантили.
//
// Используем ближайший порядковый элемент с индексом floor((n−1)·доля).
// Доля 0 даёт минимум, 1 — максимум; между ними выбирается элемент внутри ряда.

fn main() {
    let mut values = [9, 1, 7, 3, 5];
    assert!(!values.is_empty(), "для квантиля нужна непустая выборка");
    values.sort();
    for (description, fraction, expected) in [
        ("минимум", 0.0, 1),
        ("середина", 0.5, 5),
        ("три четверти", 0.75, 7),
        ("максимум", 1.0, 9),
    ] {
        assert!(
            (0.0..=1.0).contains(&fraction),
            "доля должна быть от 0 до 1"
        );
        let index = ((values.len() - 1) as f64 * fraction) as usize;
        let quantile = values[index];
        assert_eq!(quantile, expected);
        println!("{description}: доля {fraction} → {quantile}");
    }
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = (0..=100)
        .map(|i| {
            let p = i as f64 / 100.0;
            (p, [1.0, 2.0, 3.0, 4.0, 5.0][((p * 4.0) as usize).min(4)])
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Квантили выборки",
        "доля",
        "квантиль",
        &[lesson_visualization::Series {
            name: "значения 1, 2, 3, 4, 5",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
