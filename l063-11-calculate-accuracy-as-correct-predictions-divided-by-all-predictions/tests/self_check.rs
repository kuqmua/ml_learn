use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли accuracy для TP=3, FP=2, TN=4, FN=1.
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    assert!(
        (calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
            BinaryClassificationCounts {
                true_positives_as_correctly_detected_positive_cases: 3,
                false_positives_as_false_alarms_on_negative_cases: 2,
                true_negatives_as_correctly_rejected_negative_cases: 4,
                false_negatives_as_missed_positive_cases: 1,
            },
        )
        .unwrap()
            - expected)
            .abs()
            < 1e-12
    );
}
