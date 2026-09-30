#[test]
fn correct_fraction_includes_both_positive_and_negative_answers() {
    assert_eq!(
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {
            true_positives: 3,
            false_positives: 2,
            true_negatives: 4,
            false_negatives: 1
        }),
        Some(0.7)
    );
    assert_eq!(
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {
            true_positives: 0,
            false_positives: 0,
            true_negatives: 9,
            false_negatives: 1
        }),
        Some(0.9)
    );
}

#[test]
fn all_correct_all_wrong_and_undefined_without_examples() {
    assert_eq!(
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {
            true_positives: 2,
            false_positives: 0,
            true_negatives: 3,
            false_negatives: 0
        }),
        Some(1.0)
    );
    assert_eq!(
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {
            true_positives: 0,
            false_positives: 2,
            true_negatives: 0,
            false_negatives: 3
        }),
        Some(0.0)
    );
    assert_eq!(
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {
            true_positives: 0,
            false_positives: 0,
            true_negatives: 0,
            false_negatives: 0
        }),
        None
    );
}
