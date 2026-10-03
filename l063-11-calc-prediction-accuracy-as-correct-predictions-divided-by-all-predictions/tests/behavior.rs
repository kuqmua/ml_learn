use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l063_11_calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions::calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions;

#[test]
fn correct_fraction_includes_both_pos_and_neg_answers() {
    assert_eq!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 3,
                false_poss_as_false_alarms_on_neg_cases: 2,
                true_negs_as_correctly_rejected_neg_cases: 4,
                false_negs_as_missed_pos_cases: 1
            }
        ),
        Some(0.7)
    );
    assert_eq!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 0,
                false_poss_as_false_alarms_on_neg_cases: 0,
                true_negs_as_correctly_rejected_neg_cases: 9,
                false_negs_as_missed_pos_cases: 1
            }
        ),
        Some(0.9)
    );
}

#[test]
fn all_correct_all_wrong_and_undefined_without_examples() {
    assert_eq!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 2,
                false_poss_as_false_alarms_on_neg_cases: 0,
                true_negs_as_correctly_rejected_neg_cases: 3,
                false_negs_as_missed_pos_cases: 0
            }
        ),
        Some(1.0)
    );
    assert_eq!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 0,
                false_poss_as_false_alarms_on_neg_cases: 2,
                true_negs_as_correctly_rejected_neg_cases: 0,
                false_negs_as_missed_pos_cases: 3
            }
        ),
        Some(0.0)
    );
    assert_eq!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 0,
                false_poss_as_false_alarms_on_neg_cases: 0,
                true_negs_as_correctly_rejected_neg_cases: 0,
                false_negs_as_missed_pos_cases: 0
            }
        ),
        None
    );
}
