// Урок 22.2. Формы тензоров.
//
// Матрицы (a,b) и (c,d) можно умножить, только если b=c; ответ имеет форму (a,d).

fn main() {
    for (description, left_shape, right_shape, expected) in [
        ("совместимые формы", (2, 3), (3, 4), Some((2, 4))),
        ("квадратные матрицы", (2, 2), (2, 2), Some((2, 2))),
        ("несовместимые формы", (2, 3), (2, 4), None),
    ] {
        let result_shape = if left_shape.1 == right_shape.0 {
            Some((left_shape.0, right_shape.1))
        } else {
            None
        };
        assert_eq!(result_shape, expected);
        println!("{description}: {left_shape:?} × {right_shape:?} → {result_shape:?}");
    }
}
