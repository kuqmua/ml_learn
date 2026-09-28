//! Вычисления и примеры урока part-031-lesson-06-variance.

/// Выборочная дисперсия использует среднее из урока 06.1.
pub fn sample_variance(values: &[f64]) -> Result<f64, &'static str> {
    if values.len() < 2 {
        return Err("для выборочной дисперсии нужны хотя бы два значения");
    }
    let average = lesson_029::mean(values)?;
    let mut squared_deviation_sum = 0.0;
    for &value in values {
        let deviation = value - average;
        squared_deviation_sum += deviation * deviation;
    }
    Ok(squared_deviation_sum / (values.len() - 1) as f64)
}

// Урок 06.3. Выборочная дисперсия.
//
// Общая функция использует среднее из урока 06.1. При одинаковых значениях разброс равен нулю;
// для выборочной оценки нужны хотя бы два значения.

pub fn run() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("все значения одинаковы", &[4.0, 4.0, 4.0], 0.0),
        ("умеренный разброс", &[2.0, 4.0, 6.0], 4.0),
        ("значения раздвинули", &[0.0, 4.0, 8.0], 16.0),
    ];
    for (description, values, expected) in cases {
        let variance =
            crate::sample_variance(values).expect("для этой выборки дисперсия определена");
        assert_eq!(variance, expected);
        println!("{description}: {values:?} → дисперсия {variance}");
    }
    let error = crate::sample_variance(&[4.0]).expect_err("одного значения недостаточно");
    println!("одно значение: {error}");
}
