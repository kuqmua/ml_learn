// Урок 11.1. Подсчёт верных и ошибочных положительных и отрицательных прогнозов.
// Связь с принятой терминологией: Матрица ошибок бинарной классификации по истинным и прогнозным меткам.
// Зачем здесь эта тема: Одного accuracy мало: нужно различать четыре исхода бинарного прогноза.
// Почему код устроен так: Считаем TP, FP, TN и FN по парам меток, прежде чем выводить следующие
//   метрики.
// Представь: Из 100 верных ответов можно не заметить, что модель пропускает почти все редкие
//   положительные случаи.
//
// Каждая пара «истина, прогноз» попадает ровно в одну из четырёх ячеек.
// Эти счётчики затем повторно используются в precision, recall, F1 и сводной практике.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::{
    BinaryClassificationCounts, count_binary_classification_outcomes_from_targets_and_predictions,
};

fn main() {
    let truth: [bool; 4] = [true, false, true, false];
    let predicted: [bool; 4] = [true, true, false, false];
    let counts: BinaryClassificationCounts =
        count_binary_classification_outcomes_from_targets_and_predictions(&truth, &predicted)
            .expect("число ответов и прогнозов должно совпадать");
    for index in 0..truth.len() {
        let _: &str = match (truth[index], predicted[index]) {
            (true, true) => "TP: верно найден положительный класс",

            (false, true) => "FP: ложная тревога",

            (true, false) => "FN: положительный класс пропущен",

            (false, false) => "TN: верно найден отрицательный класс",
        };
        let _ = (&(truth[index]), &(predicted[index]));
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

    plot_counts_of_correct_and_incorrect_class_predictions(counts);
}

// Строим график по результатам урока.
fn plot_counts_of_correct_and_incorrect_class_predictions(counts: BinaryClassificationCounts) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Матрица ошибок: исходы",
        "количество",
        &[
            ("TP", counts.true_positives as f64),
            ("FP", counts.false_positives as f64),
            ("TN", counts.true_negatives as f64),
            ("FN", counts.false_negatives as f64),
        ],
    )
    .expect("не удалось сохранить график");
}
