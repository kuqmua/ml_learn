// Урок 01.3. Евклидова норма L2.
//
// Что изучаем: длина вектора — корень из суммы квадратов координат.
// Смена знаков длину не меняет; длина нулевого вектора равна нулю.

fn main() {
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("обычный вектор", [3.0, 4.0], 5.0),
        ("сменили знаки", [-3.0, -4.0], 5.0),
        ("вдвое длиннее", [6.0, 8.0], 10.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (description, vector, expected) in cases {
        // В библиотеке длина строится на вычислении из первого урока.
        let length = part_003_lesson_01_l2_norm::l2_norm(&vector);
        assert!((length - expected).abs() < 1e-10);
        println!("{description}: {vector:?} → L2 = {length}");
    }
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = (-50..=50)
        .map(|i| {
            let x = i as f64 / 10.0;
            (x, part_003_lesson_01_l2_norm::l2_norm(&[x, 4.0]))
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Норма L2",
        "первая координата",
        "L2",
        &[lesson_visualization::Series {
            name: "вектор [x, 4]",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
