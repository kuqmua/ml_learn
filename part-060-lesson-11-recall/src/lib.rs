//! Вычисления и примеры урока part-060-lesson-11-recall.

/// Доля найденных среди действительно положительных объектов.
pub fn recall(counts: lesson_058::Counts) -> Option<f64> {
    let actual_positives = counts.true_positives + counts.false_negatives;
    if actual_positives == 0 {
        None
    } else {
        Some(counts.true_positives as f64 / actual_positives as f64)
    }
}

// Урок 11.3. Полнота положительного класса (recall).
//
// Используем те же четыре счётчика. Если положительных объектов нет,
// значение здесь считаем неопределённым.

pub fn run() {
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
        let recall = crate::recall(counts);
        assert_eq!(recall, expected);
        println!("{description}: recall={recall:?}");
    }
}
