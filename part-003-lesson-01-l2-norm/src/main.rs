// Урок 01.3. Евклидова норма L2.
//
// Что изучаем: длина вектора — корень из суммы квадратов координат.
// Смена знаков длину не меняет; длина нулевого вектора равна нулю.

fn main() {
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("обычный вектор", [3.0, 4.0], 5.0),
        ("сменили знаки", [-3.0, -4.0], 5.0),
        ("вдвое длиннее", [6.0, 8.0], 10.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (description, vector, expected) in cases {
        let mut squared_length = 0.0;
        for coordinate in vector {
            squared_length += coordinate * coordinate;
        }
        // Нулевой случай обрабатываем отдельно, чтобы не делить на ноль в методе Ньютона.
        let mut length = squared_length;
        if squared_length > 0.0 {
            for _ in 0..80 {
                length = (length + squared_length / length) / 2.0;
            }
        }
        assert!((length - expected).abs() < 1e-10);
        println!("{description}: {vector:?} → L2 = {length}");
    }
}
