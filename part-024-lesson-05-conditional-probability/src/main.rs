// Урок 05.1. Условная вероятность.
//
// P(A|B) — доля случаев A среди случаев B. Когда B не встречается, знаменатель равен нулю
// и условная вероятность на этих данных не определена.

fn main() {
    for (description, positive_tests, sick_and_positive, expected) in [
        ("часть положительных тестов верна", 20.0, 8.0, Some(0.4)),
        ("все положительные тесты верны", 20.0, 20.0, Some(1.0)),
        ("ни один положительный тест не верен", 20.0, 0.0, Some(0.0)),
        ("положительных тестов не было", 0.0, 0.0, None),
    ] {
        assert!(positive_tests >= 0.0 && sick_and_positive >= 0.0);
        assert!(
            sick_and_positive <= positive_tests,
            "совместных случаев не может быть больше всех случаев B"
        );
        let probability = if positive_tests == 0.0 {
            None
        } else {
            Some(sick_and_positive / positive_tests)
        };
        assert_eq!(probability, expected);
        println!("{description}: P(болен | тест положительный)={probability:?}");
    }
}
