// Сводная практика 06. Выборочная статистика.
//
// Разбираем CSV и объединяем вычисления среднего и дисперсии из общей библиотеки.
// Медиану находим после сортировки. Пустые строки пропускаем, неверное число сообщаем явно.

fn main() {
    const SAMPLE_CSV: &str = "value\n2\n4\n\n6\n8\n";
    let mut values = Vec::new();
    for (line_index, line) in SAMPLE_CSV.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let value: f64 = line
            .parse()
            .unwrap_or_else(|_| panic!("строка {}: не число", line_index + 1));
        values.push(value);
    }
    assert!(!values.is_empty(), "для статистики нужны числовые значения");
    values.sort_by(f64::total_cmp);

    let mean = lesson_029::mean(&values).unwrap();
    let sample_variance = lesson_031::sample_variance(&values).unwrap();
    let middle = values.len() / 2;
    let median = if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    };
    println!(
        "n={}, mean={mean}, median={median}, sample variance={sample_variance:.3}",
        values.len()
    );
}
