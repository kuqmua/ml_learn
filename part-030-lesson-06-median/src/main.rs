// Урок 06.2. Медиана.
//
// У нечётного набора берём средний элемент, у чётного — среднее двух центральных.
// Выброс меняет среднее арифметическое гораздо сильнее, чем медиану.

fn main() {
    let cases: [(&str, &[f64], f64); 4] = [
        ("нечётное число значений", &[2.0, 4.0, 6.0], 4.0),
        ("чётное число значений", &[2.0, 4.0, 6.0, 8.0], 5.0),
        ("сильный выброс", &[2.0, 4.0, 6.0, 100.0], 5.0),
        ("одно значение", &[7.0], 7.0),
    ];
    for (description, source, expected) in cases {
        assert!(!source.is_empty(), "медиана пустого набора не определена");
        let mut values = source.to_vec();
        values.sort_by(f64::total_cmp);
        let middle = values.len() / 2;
        let median = if values.len() % 2 == 0 {
            (values[middle - 1] + values[middle]) / 2.0
        } else {
            values[middle]
        };
        assert_eq!(median, expected);
        println!("{description}: {values:?} → медиана {median}");
    }
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = [(1.0, 1.0), (2.0, 3.0), (3.0, 7.0)].to_vec();
    let chart_points_1: Vec<(f64, f64)> = [(1.0, 3.0), (3.0, 3.0)].to_vec();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Медиана и отдельные значения",
        "индекс",
        "значение",
        &[
            lesson_visualization::Series {
                name: "наблюдения",
                points: &chart_points_0,
            },
            lesson_visualization::Series {
                name: "медиана",
                points: &chart_points_1,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
