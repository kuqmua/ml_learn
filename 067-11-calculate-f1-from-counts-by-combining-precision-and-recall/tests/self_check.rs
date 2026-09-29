use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts as Counts;
use l067_11_calculate_f1_from_counts_by_combining_precision_and_recall::calculate_f1_score_from_counts_by_combining_precision_and_recall_as_twice_product_over_sum as operation;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли F1 для TP=3, FP=1, TN=4, FN=2.
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    let actual = operation(Counts {
        true_positives: 3,
        false_positives: 1,
        true_negatives: 4,
        false_negatives: 2,
    })
    .unwrap();
    assert!((actual - expected).abs() < 1e-12);
}
