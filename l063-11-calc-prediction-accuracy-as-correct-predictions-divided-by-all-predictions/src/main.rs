// Урок 063. Считаем долю правильных ответов среди всех прогнозов.
// Если из 10 прогнозов верны 8, получаем 8/10 = 0.8, то есть 80%.
// Значение 1 означает, что верно всё; 0 — что всё неверно.
// Если прогнозов нет, долю посчитать нельзя: функция возвращает None.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l063_11_calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions::calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions;

fn main() {
    let counts = BinaryClassificationCounts {
        true_poss_as_correctly_detected_pos_cases: 0,
        false_poss_as_false_alarms_on_neg_cases: 0,
        true_negs_as_correctly_rejected_neg_cases: 9,
        false_negs_as_missed_pos_cases: 1,
    };

    assert_eq!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(counts),
        Some(0.9)
    );

    let empty = BinaryClassificationCounts {
        true_poss_as_correctly_detected_pos_cases: 0,
        false_poss_as_false_alarms_on_neg_cases: 0,
        true_negs_as_correctly_rejected_neg_cases: 0,
        false_negs_as_missed_pos_cases: 0,
    };
    let _ = &(calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(empty));
}

// Чему учит этот урок:
// Учимся считать долю всех верных прогнозов и учитывать пустой набор.
// Пример с девятью верными отказами и одним пропуском показывает: высокая доля верных ответов
// может скрывать ненайденный класс.
