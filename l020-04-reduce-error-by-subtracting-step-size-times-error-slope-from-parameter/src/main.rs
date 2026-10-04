// Урок 020. Обновлять параметр, вычитая произведение размера шага и производной ошибки.
// Вычисляем результат для разных шагов, чтобы увидеть, почему слишком большой шаг может увеличить
// ошибку.

fn main() {
    let parameter: f64 = 0.0;
    let loss = |value: f64| (value - 3.0).powi(2);
    let slope = 2.0 * (parameter - 3.0);
    for rate in [0.1, 1.0, 2.0] {
        let updated = parameter - rate * slope;
        let before = loss(parameter);
        let after = loss(updated);
        println!(
            "Размер шага={rate}: параметр {parameter} -> {updated}; ошибка {before} -> {after}"
        );
        if rate == 0.1 {
            assert!(after < before);
        }
        if rate == 1.0 {
            assert_eq!(after, before);
        }
        if rate == 2.0 {
            assert!(after > before);
        }
    }
}

// Чему учит этот урок:
// Учимся обновлять параметр, вычитая произведение размера шага и производной ошибки.
// Вычисляем результат для разных шагов, чтобы увидеть, почему слишком большой шаг может увеличить
// ошибку.
