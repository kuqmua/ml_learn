// Урок 11.2. Точность положительных прогнозов (precision).
//
// Делим число верных положительных прогнозов на число всех положительных прогнозов.
// Если положительных прогнозов не было, значение метрики здесь считаем неопределённым.

fn main() {
    for (description, true_positive, false_positive, expected) in [
        ("все положительные прогнозы верны", 8.0, 0.0, Some(1.0)),
        ("часть прогнозов ошибочна", 8.0, 2.0, Some(0.8)),
        ("все положительные прогнозы ошибочны", 0.0, 2.0, Some(0.0)),
        ("положительных прогнозов нет", 0.0, 0.0, None),
    ] {
        let count = true_positive + false_positive;
        let precision = if count == 0.0 {
            None
        } else {
            Some(true_positive / count)
        };
        assert_eq!(precision, expected);
        println!("{description}: precision={precision:?}");
    }
}
