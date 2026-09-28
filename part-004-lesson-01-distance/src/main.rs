// Урок 01.4. Евклидово расстояние между точками.
//
// Что изучаем: разности по каждой координате возводим в квадрат, складываем и извлекаем корень.
// Для совпадающих точек ответ 0. Порядок точек не влияет на расстояние.

fn main() {
    let cases: [(&str, &[f64], &[f64], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    for (description, first_point, second_point, expected) in cases {
        // Общая функция проверяет размерности и вычисляет расстояние.
        let distance = part_004_lesson_01_distance::distance(first_point, second_point)
            .expect("точки в этом примере имеют одинаковую размерность");
        assert!((distance - expected).abs() < 1e-10);
        println!("{description}: {first_point:?} и {second_point:?} → {distance}");
    }
    let first_point = [0.0, 0.0];
    let too_short = [3.0];
    let error = part_004_lesson_01_distance::distance(&first_point, &too_short)
        .expect_err("точки разной размерности нужно отклонить");
    println!("разная размерность: {error}");
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = (-50..=50)
        .map(|i| {
            let x = i as f64 / 10.0;
            (
                x,
                part_004_lesson_01_distance::distance(&[0.0, 0.0], &[x, 4.0]).unwrap(),
            )
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Расстояние до начала координат",
        "x точки [x, 4]",
        "расстояние",
        &[lesson_visualization::Series {
            name: "расстояние",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
