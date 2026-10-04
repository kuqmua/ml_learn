// Урок 069. Оценивать, насколько рано нужные объекты появляются в упорядоченном списке.
// Суммируем точность на позициях находок, чтобы получить среднюю точность поиска — average
// precision.

fn main() {
    let average_precision = |ranked: &[bool]| {
        let count = ranked.iter().filter(|&&v| v).count();
        assert!(count > 0);
        let mut found = 0;
        let mut sum = 0.0;
        for (rank, &target) in ranked.iter().enumerate() {
            if target {
                found += 1;
                sum += found as f64 / (rank + 1) as f64;
            }
        }
        sum / count as f64
    };
    let good = [true, true, false, false];
    let bad = [false, false, true, true];
    println!(
        "Нужные объекты в начале: {}; в конце: {}",
        average_precision(&good),
        average_precision(&bad)
    );
    assert_eq!(average_precision(&good), 1.0);
    assert!(average_precision(&bad) < 0.5);
}

// Чему учит этот урок:
// Учимся оценивать, насколько рано нужные объекты появляются в упорядоченном списке.
// Суммируем точность на позициях находок, чтобы получить среднюю точность поиска — average
// precision.
