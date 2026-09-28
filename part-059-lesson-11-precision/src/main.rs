// Урок 11.2. Точность положительных прогнозов (precision).
//
// Используем четыре счётчика из урока 11.1. Если положительных прогнозов нет,
// значение здесь считаем неопределённым.

fn main() {
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
        let precision = part_059_lesson_11_precision::precision(counts);
        assert_eq!(precision, expected);
        println!("{description}: precision={precision:?}");
    }
}
