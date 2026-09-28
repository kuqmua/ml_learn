// Урок 06.3. Выборочная дисперсия.
//
// Общая функция использует среднее из урока 06.1. При одинаковых значениях разброс равен нулю;
// для выборочной оценки нужны хотя бы два значения.

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("все значения одинаковы", &[4.0, 4.0, 4.0], 0.0),
        ("умеренный разброс", &[2.0, 4.0, 6.0], 4.0),
        ("значения раздвинули", &[0.0, 4.0, 8.0], 16.0),
    ];
    for (description, values, expected) in cases {
        let variance = part_031_lesson_06_variance::sample_variance(values)
            .expect("для этой выборки дисперсия определена");
        assert_eq!(variance, expected);
        println!("{description}: {values:?} → дисперсия {variance}");
    }
    let error = part_031_lesson_06_variance::sample_variance(&[4.0])
        .expect_err("одного значения недостаточно");
    println!("одно значение: {error}");
}
