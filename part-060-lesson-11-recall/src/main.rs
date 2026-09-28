// Урок 11.3. Полнота положительного класса (recall).
//
// Используем те же четыре счётчика. Если положительных объектов нет,
// значение здесь считаем неопределённым.

fn main() {
    for (description, true_positives, false_negatives, expected) in [
        ("найдены все", 8, 0, Some(1.0)),
        ("найдены не все", 8, 4, Some(2.0 / 3.0)),
        ("не найден ни один", 0, 4, Some(0.0)),
        ("положительных объектов нет", 0, 0, None),
    ] {
        let counts = lesson_058::Counts {
            true_positives,
            false_positives: 0,
            true_negatives: 0,
            false_negatives,
        };
        let recall = part_060_lesson_11_recall::recall(counts);
        assert_eq!(recall, expected);
        println!("{description}: recall={recall:?}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (0..=10)
        .map(|n| (n as f64, 2.0 / (2.0 + n as f64)))
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Recall при фиксированном TP=2",
        "FN",
        "recall",
        &[lesson_visualization::Series {
            name: "recall",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
