#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли accuracy для TP=3, FP=2, TN=4, FN=1.
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    let actual = l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {
        true_positives: 3,
        false_positives: 2,
        true_negatives: 4,
        false_negatives: 1,
    })
    .unwrap();
    assert!((actual - expected).abs() < 1e-12);
}
