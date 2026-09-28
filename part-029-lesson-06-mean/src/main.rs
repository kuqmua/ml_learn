// Урок 06.1. Среднее арифметическое.
//
// Складываем значения и делим на их количество. Для одного значения среднее равно ему,
// отрицательные числа могут уменьшить среднее, а для пустого набора делить не на что.

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("несколько положительных", &[2.0, 4.0, 6.0], 4.0),
        ("одно значение", &[7.0], 7.0),
        ("значения разных знаков", &[-2.0, 2.0], 0.0),
    ];
    for (description, values, expected) in cases {
        assert!(!values.is_empty(), "среднее пустого набора не определено");
        let mut sum = 0.0;
        for &value in values {
            sum += value;
        }
        let mean = sum / values.len() as f64;
        assert_eq!(mean, expected);
        println!("{description}: {values:?} → среднее {mean}");
    }
    let empty: [f64; 0] = [];
    if empty.is_empty() {
        println!("пустой набор: среднее не определено");
    }
}
