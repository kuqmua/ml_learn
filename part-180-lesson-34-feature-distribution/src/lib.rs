//! Вычисления и примеры урока part-180-lesson-34-feature-distribution.

// Урок 34.3. Распределение признака.
//
// Считаем значения ниже 0.5 и значения от 0.5. Сравнение старой и новой выборки
// показывает, переместилась ли масса распределения между интервалами.

pub fn run() {
    let cases = [
        ("эталон", [0.1, 0.2, 0.8, 0.9], [2, 2]),
        ("без сдвига", [0.2, 0.3, 0.7, 0.8], [2, 2]),
        ("сдвиг к большим значениям", [0.6, 0.7, 0.8, 0.9], [0, 4]),
    ];
    for (description, values, expected) in cases {
        let mut bins = [0; 2];
        for value in values {
            let index = if value < 0.5 { 0 } else { 1 };
            bins[index] += 1;
        }
        assert_eq!(bins, expected);
        println!("{description}: {values:?} → частоты {bins:?}");
    }
}
