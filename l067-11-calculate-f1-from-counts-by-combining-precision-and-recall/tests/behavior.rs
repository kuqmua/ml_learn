use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l067_11_calculate_f1_from_counts_by_combining_precision_and_recall::calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better;

#[test]
fn f1_combines_precision_and_recall_and_ignores_true_negatives() {
    for true_negatives_as_correctly_rejected_negative_cases in [0, 4, 1000] {
        assert!(check_f64_eq_1e_minus_12(calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(BinaryClassificationCounts {
            true_positives_as_correctly_detected_positive_cases: 3,
            false_positives_as_false_alarms_on_negative_cases: 1,
            true_negatives_as_correctly_rejected_negative_cases,
            false_negatives_as_missed_positive_cases: 2,
        })
        .unwrap(), 2.0 / 3.0));
    }
    assert_eq!(
        calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(
            BinaryClassificationCounts {
                true_positives_as_correctly_detected_positive_cases: 3,
                false_positives_as_false_alarms_on_negative_cases: 0,
                true_negatives_as_correctly_rejected_negative_cases: 5,
                false_negatives_as_missed_positive_cases: 0
            }
        ),
        Some(1.0)
    );
}

#[test]
fn undefined_components_or_zero_metric_sum_return_none() {
    for counts in [
        BinaryClassificationCounts {
            true_positives_as_correctly_detected_positive_cases: 0,
            false_positives_as_false_alarms_on_negative_cases: 0,
            true_negatives_as_correctly_rejected_negative_cases: 4,
            false_negatives_as_missed_positive_cases: 2,
        },
        BinaryClassificationCounts {
            true_positives_as_correctly_detected_positive_cases: 0,
            false_positives_as_false_alarms_on_negative_cases: 2,
            true_negatives_as_correctly_rejected_negative_cases: 4,
            false_negatives_as_missed_positive_cases: 0,
        },
        BinaryClassificationCounts {
            true_positives_as_correctly_detected_positive_cases: 0,
            false_positives_as_false_alarms_on_negative_cases: 2,
            true_negatives_as_correctly_rejected_negative_cases: 4,
            false_negatives_as_missed_positive_cases: 3,
        },
        BinaryClassificationCounts {
            true_positives_as_correctly_detected_positive_cases: 0,
            false_positives_as_false_alarms_on_negative_cases: 0,
            true_negatives_as_correctly_rejected_negative_cases: 0,
            false_negatives_as_missed_positive_cases: 0,
        },
    ] {
        assert_eq!(calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(counts), None);
    }
}
