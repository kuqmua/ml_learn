use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l063_11_calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong::calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли accuracy для TP=3, FP=2, TN=4, FN=1.
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    assert!(
        check_f64_eq_1e_minus_12(calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 3,
                false_poss_as_false_alarms_on_neg_cases: 2,
                true_negs_as_correctly_rejected_neg_cases: 4,
                false_negs_as_missed_pos_cases: 1,
            },
        )
        .unwrap(), expected)
    );
}
