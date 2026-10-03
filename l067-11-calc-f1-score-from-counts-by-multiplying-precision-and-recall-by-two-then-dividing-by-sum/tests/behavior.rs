use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l067_11_calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum::calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum;

#[test]
fn f1_combines_precision_and_recall_and_ignores_true_negs() {
    for true_negs_as_correctly_rejected_neg_cases in [0, 4, 1000] {
        assert!(check_f64_eq_1e_minus_12(calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 3,
            false_poss_as_false_alarms_on_neg_cases: 1,
            true_negs_as_correctly_rejected_neg_cases,
            false_negs_as_missed_pos_cases: 2,
        })
        .unwrap(), 2.0 / 3.0));
    }
    assert_eq!(
        calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
            BinaryClassificationCounts {
                true_poss_as_correctly_detected_pos_cases: 3,
                false_poss_as_false_alarms_on_neg_cases: 0,
                true_negs_as_correctly_rejected_neg_cases: 5,
                false_negs_as_missed_pos_cases: 0
            }
        ),
        Some(1.0)
    );
}

#[test]
fn undefined_components_or_zero_metric_sum_return_none() {
    for counts in [
        BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 0,
            false_poss_as_false_alarms_on_neg_cases: 0,
            true_negs_as_correctly_rejected_neg_cases: 4,
            false_negs_as_missed_pos_cases: 2,
        },
        BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 0,
            false_poss_as_false_alarms_on_neg_cases: 2,
            true_negs_as_correctly_rejected_neg_cases: 4,
            false_negs_as_missed_pos_cases: 0,
        },
        BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 0,
            false_poss_as_false_alarms_on_neg_cases: 2,
            true_negs_as_correctly_rejected_neg_cases: 4,
            false_negs_as_missed_pos_cases: 3,
        },
        BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 0,
            false_poss_as_false_alarms_on_neg_cases: 0,
            true_negs_as_correctly_rejected_neg_cases: 0,
            false_negs_as_missed_pos_cases: 0,
        },
    ] {
        assert_eq!(calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(counts), None);
    }
}
