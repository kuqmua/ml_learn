// Доля верных прогнозов: число правильных ответов, делённое на общее число.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l063_11_calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong::calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong;

fn main() {
    let counts = BinaryClassificationCounts {
        true_poss_as_correctly_detected_pos_cases: 0,
        false_poss_as_false_alarms_on_neg_cases: 0,
        true_negs_as_correctly_rejected_neg_cases: 9,
        false_negs_as_missed_pos_cases: 1,
    };

    assert_eq!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(counts),
        Some(0.9)
    );

    let empty = BinaryClassificationCounts {
        true_poss_as_correctly_detected_pos_cases: 0,
        false_poss_as_false_alarms_on_neg_cases: 0,
        true_negs_as_correctly_rejected_neg_cases: 0,
        false_negs_as_missed_pos_cases: 0,
    };
    let _ =
        &(calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(empty));
}
