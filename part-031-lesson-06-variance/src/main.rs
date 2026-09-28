// Урок 06.3. Выборочная дисперсия.
//
// Общая функция использует среднее из урока 06.1. При одинаковых значениях разброс равен нулю;
// для выборочной оценки нужны хотя бы два значения.

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("все значения одинаковы", &[4.0, 4.0, 4.0], 0.0),
        ("умеренный разброс", &[2.0, 4.0, 6.0], 4.0),
        ("значения раздвинули", &[0.0, 4.0, 8.0], 16.0),
    ];
    for (description, values, expected) in cases {
        let variance = part_031_lesson_06_variance::sample_variance(values)
            .expect("для этой выборки дисперсия определена");
        assert_eq!(variance, expected);
        println!("{description}: {values:?} → дисперсия {variance}");
    }
    let error = part_031_lesson_06_variance::sample_variance(&[4.0])
        .expect_err("одного значения недостаточно");
    println!("одно значение: {error}");
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = (0..=80)
        .map(|i| {
            let x = i as f64 / 10.0;
            (x, (x - 4.0) * (x - 4.0))
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Разброс относительно среднего",
        "значение",
        "квадрат отклонения",
        &[lesson_visualization::Series {
            name: "среднее=4",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
