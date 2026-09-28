//! Вычисления и примеры урока part-165-lesson-31-distribution-shift.

// Урок 31.1. Сдвиг распределения.
//
// Сравнение средних — первый сигнал: при похожих данных разница мала, при сдвиге растёт.
// Совпадение средних само по себе не доказывает совпадения распределений.

pub fn run() {
    let reference = [1.0, 2.0, 3.0];
    let cases = [
        ("без сдвига среднего", [3.0, 2.0, 1.0], 0.0),
        ("сдвиг к большим значениям", [5.0, 6.0, 7.0], 4.0),
        ("то же среднее, другой разброс", [0.0, 2.0, 4.0], 0.0),
    ];
    assert!(!reference.is_empty());
    let reference_mean = lesson_029::mean(&reference).unwrap();
    for (description, current, expected_difference) in cases {
        assert!(!current.is_empty());
        let current_mean = lesson_029::mean(&current).unwrap();
        let difference = current_mean - reference_mean;
        assert_eq!(difference, expected_difference);
        println!(
            "{description}: эталон={reference_mean}, новые данные={current_mean}, разница={difference}"
        );
    }
}
