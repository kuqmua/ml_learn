use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong;

#[test]
fn correct_fraction_includes_both_positive_and_negative_answers() {
    assert_eq!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
            BinaryClassificationCounts {
                true_positives_as_correctly_detected_positive_cases: 3,
                false_positives_as_false_alarms_on_negative_cases: 2,
                true_negatives_as_correctly_rejected_negative_cases: 4,
                false_negatives_as_missed_positive_cases: 1
            }
        ),
        Some(0.7)
    );
    assert_eq!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
            BinaryClassificationCounts {
                true_positives_as_correctly_detected_positive_cases: 0,
                false_positives_as_false_alarms_on_negative_cases: 0,
                true_negatives_as_correctly_rejected_negative_cases: 9,
                false_negatives_as_missed_positive_cases: 1
            }
        ),
        Some(0.9)
    );
}

#[test]
fn all_correct_all_wrong_and_undefined_without_examples() {
    assert_eq!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
            BinaryClassificationCounts {
                true_positives_as_correctly_detected_positive_cases: 2,
                false_positives_as_false_alarms_on_negative_cases: 0,
                true_negatives_as_correctly_rejected_negative_cases: 3,
                false_negatives_as_missed_positive_cases: 0
            }
        ),
        Some(1.0)
    );
    assert_eq!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
            BinaryClassificationCounts {
                true_positives_as_correctly_detected_positive_cases: 0,
                false_positives_as_false_alarms_on_negative_cases: 2,
                true_negatives_as_correctly_rejected_negative_cases: 0,
                false_negatives_as_missed_positive_cases: 3
            }
        ),
        Some(0.0)
    );
    assert_eq!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
            BinaryClassificationCounts {
                true_positives_as_correctly_detected_positive_cases: 0,
                false_positives_as_false_alarms_on_negative_cases: 0,
                true_negatives_as_correctly_rejected_negative_cases: 0,
                false_negatives_as_missed_positive_cases: 0
            }
        ),
        None
    );
}
