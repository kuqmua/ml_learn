// Урок 11.1. Матрица ошибок классификации.
//
// Каждая пара «истина, прогноз» попадает ровно в одну из четырёх ячеек.
// Эти счётчики затем повторно используются в precision, recall, F1 и сводной практике.

fn main() {
    let truth = [true, false, true, false];
    let predicted = [true, true, false, false];
    let counts = part_058_lesson_11_confusion_matrix::count_outcomes(&truth, &predicted)
        .expect("у каждого ответа есть прогноз");
    for index in 0..truth.len() {
        let description = match (truth[index], predicted[index]) {
            (true, true) => "TP: верно найден положительный класс",
            (false, true) => "FP: ложная тревога",
            (true, false) => "FN: положительный класс пропущен",
            (false, false) => "TN: верно найден отрицательный класс",
        };
        println!(
            "истина={}, прогноз={} → {description}",
            truth[index], predicted[index]
        );
    }
    assert_eq!(
        (
            counts.true_positives,
            counts.false_positives,
            counts.false_negatives,
            counts.true_negatives
        ),
        (1, 1, 1, 1)
    );
    println!("итоговые счётчики: {counts:?}");
}
