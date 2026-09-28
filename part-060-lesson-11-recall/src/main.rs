// Урок 11.3. Полнота положительного класса (recall).
//
// Делим число найденных положительных объектов на число всех действительно положительных.
// Если положительных объектов нет, значение метрики здесь считаем неопределённым.

fn main() {
    for (description, true_positive, false_negative, expected) in [
        ("найдены все", 8.0, 0.0, Some(1.0)),
        ("найдены не все", 8.0, 4.0, Some(2.0 / 3.0)),
        ("не найден ни один", 0.0, 4.0, Some(0.0)),
        ("положительных объектов нет", 0.0, 0.0, None),
    ] {
        let count = true_positive + false_negative;
        let recall = if count == 0.0 {
            None
        } else {
            Some(true_positive / count)
        };
        assert_eq!(recall, expected);
        println!("{description}: recall={recall:?}");
    }
}
