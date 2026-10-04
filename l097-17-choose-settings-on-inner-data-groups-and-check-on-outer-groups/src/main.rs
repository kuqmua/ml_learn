// Урок 097. Вычислять внутренние оценки нескольких настроек, выбирать одну и проверять её на внешнем
// наборе.
// Внешние правильные ответы используются только после выбора числа соседей.

fn main() {
    fn predict(training: &[(f64, bool)], x: f64, k: usize) -> bool {
        let mut neighbors = training.to_vec();
        neighbors.sort_by(|a, b| (a.0 - x).abs().total_cmp(&(b.0 - x).abs()));
        neighbors[..k].iter().filter(|v| v.1).count() * 2 > k
    }
    let training = [
        (0.0, false),
        (1.0, false),
        (2.0, false),
        (3.0, true),
        (4.0, true),
        (5.0, true),
    ];
    let inner_validation = [(0.5, false), (2.7, true), (4.5, true)];
    let outer_test = [(0.2, false), (3.5, true)];
    let mut best = (0, 0);
    for k in [1, 3, 5] {
        let correct = inner_validation
            .iter()
            .filter(|&&(x, y)| predict(&training, x, k) == y)
            .count();
        println!(
            "Внутренняя проверка: k={k}, верных={correct}/{}",
            inner_validation.len()
        );
        if best.0 == 0 || correct > best.1 {
            best = (k, correct);
        }
    }
    // Настройка уже выбрана. Только теперь используем внешние правильные ответы.
    let correct = outer_test
        .iter()
        .filter(|&&(x, y)| predict(&training, x, best.0) == y)
        .count();
    println!(
        "Выбрано k={}; внешняя проверка={correct}/{}",
        best.0,
        outer_test.len()
    );
    assert_eq!(best.0, 1);
    assert_eq!(correct, 2);
}

// Чему учит этот урок:
// Учимся вычислять внутренние оценки нескольких настроек, выбирать одну и проверять её на внешнем
// наборе.
// Внешние правильные ответы используются только после выбора числа соседей.
