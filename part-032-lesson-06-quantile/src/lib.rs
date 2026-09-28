//! Вычисления и примеры урока part-032-lesson-06-quantile.

// Урок 06.4. Квантили.
//
// Используем ближайший порядковый элемент с индексом floor((n−1)·доля).
// Доля 0 даёт минимум, 1 — максимум; между ними выбирается элемент внутри ряда.

pub fn run() {
    let mut values = [9, 1, 7, 3, 5];
    assert!(!values.is_empty(), "для квантиля нужна непустая выборка");
    values.sort();
    for (description, fraction, expected) in [
        ("минимум", 0.0, 1),
        ("середина", 0.5, 5),
        ("три четверти", 0.75, 7),
        ("максимум", 1.0, 9),
    ] {
        assert!(
            (0.0..=1.0).contains(&fraction),
            "доля должна быть от 0 до 1"
        );
        let index = ((values.len() - 1) as f64 * fraction) as usize;
        let quantile = values[index];
        assert_eq!(quantile, expected);
        println!("{description}: доля {fraction} → {quantile}");
    }
}
