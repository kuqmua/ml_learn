//! Вычисления и примеры урока part-029-lesson-06-mean.

/// Среднее непустого набора.
pub fn mean(values: &[f64]) -> Result<f64, &'static str> {
    if values.is_empty() {
        return Err("для среднего нужно хотя бы одно значение");
    }
    let mut sum = 0.0;
    for &value in values {
        sum += value;
    }
    Ok(sum / values.len() as f64)
}

// Урок 06.1. Среднее арифметическое.
//
// Складываем значения и делим на их количество. Для одного значения среднее равно ему,
// отрицательные числа могут уменьшить среднее, а для пустого набора делить не на что.

pub fn run() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("несколько положительных", &[2.0, 4.0, 6.0], 4.0),
        ("одно значение", &[7.0], 7.0),
        ("значения разных знаков", &[-2.0, 2.0], 0.0),
    ];
    for (description, values, expected) in cases {
        // Общая функция среднего повторно понадобится в дисперсии и нормализации.
        let mean = crate::mean(values).expect("в этой строке есть значения");
        assert_eq!(mean, expected);
        println!("{description}: {values:?} → среднее {mean}");
    }
    let empty: [f64; 0] = [];
    let error = crate::mean(&empty).expect_err("среднее пустого набора должно быть отклонено");
    println!("пустой набор: {error}");
}
