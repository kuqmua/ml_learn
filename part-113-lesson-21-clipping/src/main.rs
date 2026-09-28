// Урок 21.4. Ограничение градиента.
//
// Значения внутри интервала [-limit, limit] не меняются. Выходящие за границу
// заменяются ближайшей границей с сохранением знака.

fn main() {
    let limit = 1.0;
    assert!(limit > 0.0);
    for (description, gradient, expected) in [
        ("слишком большой положительный", 12.0, 1.0),
        ("положительный внутри интервала", 0.5, 0.5),
        ("нулевой", 0.0, 0.0),
        ("отрицательный внутри интервала", -0.5, -0.5),
        ("слишком большой отрицательный", -12.0, -1.0),
    ] {
        let clipped = if gradient > limit {
            limit
        } else if gradient < -limit {
            -limit
        } else {
            gradient
        };
        assert_eq!(clipped, expected);
        println!("{description}: {gradient} → {clipped}");
    }
}
