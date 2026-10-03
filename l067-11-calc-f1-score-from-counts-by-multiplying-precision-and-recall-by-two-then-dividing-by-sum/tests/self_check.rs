use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l067_11_calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum::calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли F1 для TP=3, FP=1, TN=4, FN=2.
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    let actual =
        calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 3,
                false_poss_as_false_alarms_on_neg_cases: 1,
                true_negs_as_correctly_rejected_neg_cases: 4,
                false_negs_as_missed_pos_cases: 2,
            },
        )
        .unwrap();
    assert!(check_f64_eq_1e_minus_12(actual, expected));
}
