//! Урок 063. Считаем долю правильных ответов среди всех прогнозов.
//! Если из 10 прогнозов верны 8, получаем 8/10 = 0.8, то есть 80%.
//! Значение 1 означает, что верно всё; 0 — что всё неверно.
//! Если прогнозов нет, долю посчитать нельзя: функция возвращает None.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;

pub fn calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let all_prediction_count: usize = counts.true_poss_as_correctly_detected_pos_cases
        + counts.false_poss_as_false_alarms_on_neg_cases
        + counts.true_negs_as_correctly_rejected_neg_cases
        + counts.false_negs_as_missed_pos_cases;
    if all_prediction_count == 0 {
        None
    } else {
        Some(
            (counts.true_poss_as_correctly_detected_pos_cases
                + counts.true_negs_as_correctly_rejected_neg_cases) as f64
                / all_prediction_count as f64,
        )
    }
}
