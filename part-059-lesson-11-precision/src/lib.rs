//! Вычисления и примеры урока part-059-lesson-11-precision.

/// Доля верных среди положительных прогнозов.
pub fn precision(counts: lesson_058::Counts) -> Option<f64> {
    let predicted_positives = counts.true_positives + counts.false_positives;
    if predicted_positives == 0 {
        None
    } else {
        Some(counts.true_positives as f64 / predicted_positives as f64)
    }
}

// Урок 11.2. Точность положительных прогнозов (precision).
//
// Используем четыре счётчика из урока 11.1. Если положительных прогнозов нет,
// значение здесь считаем неопределённым.

pub fn run() {
    for (description, true_positives, false_positives, expected) in [
        ("все положительные прогнозы верны", 8, 0, Some(1.0)),
        ("часть прогнозов ошибочна", 8, 2, Some(0.8)),
        ("все положительные прогнозы ошибочны", 0, 2, Some(0.0)),
        ("положительных прогнозов нет", 0, 0, None),
    ] {
        let counts = lesson_058::Counts {
            true_positives,
            false_positives,
            true_negatives: 0,
            false_negatives: 0,
        };
        let precision = crate::precision(counts);
        assert_eq!(precision, expected);
        println!("{description}: precision={precision:?}");
    }
}
