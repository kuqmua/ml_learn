// Урок 11.1. Подсчёт верных и ошибочных положительных и отрицательных прогнозов.
// Зачем здесь эта тема: Одного accuracy мало: нужно различать четыре исхода бинарного прогноза.
// Почему код устроен так: Считаем TP, FP, TN и FN по парам меток, прежде чем выводить следующие
//   метрики.
// Представь: Из 100 верных ответов можно не заметить, что модель пропускает почти все редкие
//   положительные случаи.
//
// Каждая пара «истина, прогноз» попадает ровно в одну из четырёх ячеек.
// Эти счётчики затем повторно используются в precision, recall, F1 и сводной практике.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::{
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
            counts.true_poss_as_correctly_detected_pos_cases,
            counts.false_poss_as_false_alarms_on_neg_cases,
            counts.false_negs_as_missed_pos_cases,
            counts.true_negs_as_correctly_rejected_neg_cases
        ),
        (1, 1, 1, 1)
    );

    // Выполняем вычисления из примера.
    let _ = counts;
}
