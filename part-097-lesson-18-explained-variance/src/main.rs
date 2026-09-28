// Урок 18.4. Объяснённая дисперсия.
//
// Доля первой оси равна её дисперсии, делённой на общую дисперсию.
// Она лежит от 0 до 1; при нулевой общей дисперсии долю определить нельзя.

fn main() {
    let cases: [(&str, [f64; 2], Option<f64>); 4] = [
        ("первая ось сохраняет почти всё", [9.0, 1.0], Some(0.9)),
        ("оси равноправны", [5.0, 5.0], Some(0.5)),
        ("первая ось ничего не сохраняет", [0.0, 4.0], Some(0.0)),
        ("изменчивости нет", [0.0, 0.0], None),
    ];
    for (description, eigenvalues, expected) in cases {
        assert!(
            eigenvalues.iter().all(|&value| value >= 0.0),
            "дисперсия не может быть отрицательной"
        );
        let total_variance = eigenvalues[0] + eigenvalues[1];
        let explained_fraction = if total_variance == 0.0 {
            None
        } else {
            Some(eigenvalues[0] / total_variance)
        };
        assert_eq!(explained_fraction, expected);
        println!("{description}: {eigenvalues:?} → доля первой оси {explained_fraction:?}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (0..=50)
        .map(|i| {
            let l = i as f64 / 10.0;
            (l, l / (l + 1.0))
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Объяснённая дисперсия первой оси",
        "λ₁",
        "доля",
        &[lesson_visualization::Series {
            name: "λ₂=1",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
