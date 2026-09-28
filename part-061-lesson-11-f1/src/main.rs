// Урок 11.4. Мера F1.
//
// Объединяем precision и recall из двух предыдущих уроков.
// Если обе равны нулю, формула даёт 0/0, поэтому возвращаем None.

fn main() {
    for (description, precision, recall, expected) in [
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        let f1 = part_061_lesson_11_f1::f1(Some(precision), Some(recall));
        assert_eq!(f1, expected);
        println!("{description}: precision={precision}, recall={recall}, F1={f1:?}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (0..=100)
        .map(|i| {
            let r = i as f64 / 100.0;
            (
                r,
                if r == 0.0 {
                    0.0
                } else {
                    2.0 * 0.8 * r / (0.8 + r)
                },
            )
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "F1 при precision=0.8",
        "recall",
        "F1",
        &[lesson_visualization::Series {
            name: "F1",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
