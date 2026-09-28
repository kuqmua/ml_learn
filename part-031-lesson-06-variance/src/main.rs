// Урок 06.3. Выборочная дисперсия.
//
// Если все значения одинаковы, разброс равен нулю. Чем дальше значения от среднего,
// тем больше дисперсия. Для выборочной оценки нужны хотя бы два значения.

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("все значения одинаковы", &[4.0, 4.0, 4.0], 0.0),
        ("умеренный разброс", &[2.0, 4.0, 6.0], 4.0),
        ("значения раздвинули", &[0.0, 4.0, 8.0], 16.0),
    ];
    for (description, values, expected) in cases {
        assert!(
            values.len() >= 2,
            "для выборочной дисперсии нужны хотя бы два значения"
        );
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let mut squared_deviation_sum = 0.0;
        for &value in values {
            let deviation = value - mean;
            squared_deviation_sum += deviation * deviation;
        }
        let variance = squared_deviation_sum / (values.len() - 1) as f64;
        assert_eq!(variance, expected);
        println!("{description}: {values:?} → дисперсия {variance}");
    }
    let too_short = [4.0];
    if too_short.len() < 2 {
        println!("одно значение: выборочная дисперсия не определена");
    }
}
