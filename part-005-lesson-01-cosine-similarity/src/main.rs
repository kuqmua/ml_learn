// Урок 01.5. Косинусное сходство.
//
// Что изучаем: сравнение направлений независимо от длины векторов.
// 1 означает одинаковое направление, 0 — перпендикулярность, −1 — противоположное.
// Промежуточные значения показывают острый или тупой угол. Для нулевого вектора направления нет.

fn main() {
    let left = [1.0, 0.0];
    let cases: [(&str, &[f64], f64); 5] = [
        ("то же направление", &[2.0, 0.0], 1.0),
        ("острый угол", &[1.0, 1.0], 0.7071067811865475),
        ("перпендикулярные векторы", &[0.0, 2.0], 0.0),
        ("тупой угол", &[-1.0, 1.0], -0.7071067811865475),
        ("противоположные направления", &[-2.0, 0.0], -1.0),
    ];

    for (description, right, expected) in cases {
        // Числитель и длины уже изучены; общий код соединяет их в косинусное сходство.
        let similarity = part_005_lesson_01_cosine_similarity::cosine_similarity(&left, right)
            .expect("оба вектора ненулевые и одинаковой длины");
        assert!((similarity - expected).abs() < 1e-10);
        println!("{description}: {left:?} и {right:?} → {similarity:.3}");
    }

    // Показанные ниже входы не имеют косинусного сходства.
    for (description, right) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разная длина", &[1.0][..]),
    ] {
        let error = part_005_lesson_01_cosine_similarity::cosine_similarity(&left, right)
            .expect_err("этот вход должен быть отклонён");
        println!("{description}: {error}");
    }
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = (0..=180)
        .step_by(5)
        .map(|i| {
            let angle = (i as f64).to_radians();
            (
                i as f64,
                part_005_lesson_01_cosine_similarity::cosine_similarity(
                    &[1.0, 0.0],
                    &[angle.cos(), angle.sin()],
                )
                .unwrap(),
            )
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Косинусное сходство",
        "угол, градусы",
        "сходство",
        &[lesson_visualization::Series {
            name: "вектор [1, 0]",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
