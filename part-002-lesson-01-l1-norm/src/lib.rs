//! Вычисления и примеры урока part-002-lesson-01-l1-norm.

/// Складываем модули координат.
pub fn l1_norm(vector: &[f64]) -> f64 {
    let mut sum = 0.0;
    for &coordinate in vector {
        sum += if coordinate < 0.0 {
            -coordinate
        } else {
            coordinate
        };
    }
    sum
}

// Урок 01.2. Норма L1.
//
// Что изучаем: складываем модули всех координат. Отрицательное число даёт положительный вклад,
// поэтому смена знаков не меняет ответ. У нулевого вектора результат равен нулю.

pub fn run() {
    let cases = [
        ("положительные координаты", [3.0, 4.0], 7.0),
        ("смешанные знаки", [3.0, -4.0], 7.0),
        ("сменили оба знака", [-3.0, 4.0], 7.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (description, vector, expected) in cases {
        // Формула из общей библиотеки пригодится и в сводной практике.
        let l1_norm = crate::l1_norm(&vector);
        assert_eq!(l1_norm, expected);
        println!("{description}: {vector:?} → L1 = {l1_norm}");
    }
}
