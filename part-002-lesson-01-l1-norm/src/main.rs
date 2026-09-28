// Урок 01.2. Норма L1.
//
// Что изучаем: складываем модули всех координат. Отрицательное число даёт положительный вклад,
// поэтому смена знаков не меняет ответ. У нулевого вектора результат равен нулю.

fn main() {
    let cases = [
        ("положительные координаты", [3.0, 4.0], 7.0),
        ("смешанные знаки", [3.0, -4.0], 7.0),
        ("сменили оба знака", [-3.0, 4.0], 7.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (description, vector, expected) in cases {
        // Формула из общей библиотеки пригодится и в сводной практике.
        let l1_norm = part_002_lesson_01_l1_norm::l1_norm(&vector);
        assert_eq!(l1_norm, expected);
        println!("{description}: {vector:?} → L1 = {l1_norm}");
    }
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = (-50..=50)
        .map(|i| {
            let x = i as f64 / 10.0;
            (x, part_002_lesson_01_l1_norm::l1_norm(&[x, 4.0]))
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Норма L1",
        "первая координата",
        "L1",
        &[lesson_visualization::Series {
            name: "вектор [x, 4]",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
