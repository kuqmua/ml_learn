// Урок 01.2. Норма L1.
//
// Что изучаем: складываем модули всех координат. Отрицательное число даёт положительный вклад,
// поэтому смена знаков не меняет ответ. У нулевого вектора результат равен нулю.

fn main() {
    let cases = [
        ("положительные координаты", [3.0, 4.0], 7.0),
        ("смешанные знаки", [3.0, -4.0], 7.0),
        ("сменили оба знака", [-3.0, 4.0], 7.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (description, vector, expected) in cases {
        let mut l1_norm = 0.0;
        for coordinate in vector {
            // Модуль не даёт отрицательным координатам сократить положительные.
            let absolute_value = if coordinate < 0.0 {
                -coordinate
            } else {
                coordinate
            };
            l1_norm += absolute_value;
        }
        assert_eq!(l1_norm, expected);
        println!("{description}: {vector:?} → L1 = {l1_norm}");
    }
}
