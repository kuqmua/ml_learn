//! Вычисления и примеры урока part-064-lesson-11-metrics.

// Сводная практика 11. Метрики и дисбаланс классов.
//
// Объединяем четыре исхода, precision, recall и F1 из уроков 11.1–11.4.
// При редком положительном классе высокая accuracy может скрывать бесполезную модель.

pub fn run() {
    let labels = [
        false, false, false, false, false, false, false, false, false, true,
    ];
    let scores = [0.1, 0.2, 0.3, 0.1, 0.2, 0.4, 0.1, 0.3, 0.2, 0.8];
    let counts = lesson_058::count_outcomes_at_threshold(&labels, &scores, 0.5).unwrap();
    let precision = lesson_059::precision(counts);
    let recall = lesson_060::recall(counts);
    let f1 = lesson_061::f1(precision, recall);
    let accuracy = lesson_058::accuracy(counts);
    println!(
        "модель: {counts:?}, precision={precision:?}, recall={recall:?}, F1={f1:?}, accuracy={accuracy:?}"
    );

    let all_negative_scores = [0.0; 10];
    let useless =
        lesson_058::count_outcomes_at_threshold(&labels, &all_negative_scores, 0.5).unwrap();
    let useless_accuracy = lesson_058::accuracy(useless);
    let useless_recall = lesson_060::recall(useless);
    assert!(useless_accuracy.unwrap() > 0.8);
    assert_eq!(useless_recall, Some(0.0));
    println!("всегда отрицательно: accuracy={useless_accuracy:?}, recall={useless_recall:?}");
}
