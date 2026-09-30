use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l067_11_calculate_f1_from_counts_by_combining_precision_and_recall::calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum;

#[test]
fn f1_combines_precision_and_recall_and_ignores_true_negatives() {
    for true_negatives in [0, 4, 1000] {
        let value = calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(BinaryClassificationCounts {
            true_positives: 3,
            false_positives: 1,
            true_negatives,
            false_negatives: 2,
        })
        .unwrap();
        assert!((value - 2.0 / 3.0).abs() < 1e-12);
    }
    assert_eq!(
        calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
            BinaryClassificationCounts {
                true_positives: 3,
                false_positives: 0,
                true_negatives: 5,
                false_negatives: 0
            }
        ),
        Some(1.0)
    );
}

#[test]
fn undefined_components_or_zero_metric_sum_return_none() {
    for counts in [
        BinaryClassificationCounts {
            true_positives: 0,
            false_positives: 0,
            true_negatives: 4,
            false_negatives: 2,
        },
        BinaryClassificationCounts {
            true_positives: 0,
            false_positives: 2,
            true_negatives: 4,
            false_negatives: 0,
        },
        BinaryClassificationCounts {
            true_positives: 0,
            false_positives: 2,
            true_negatives: 4,
            false_negatives: 3,
        },
        BinaryClassificationCounts {
            true_positives: 0,
            false_positives: 0,
            true_negatives: 0,
            false_negatives: 0,
        },
    ] {
        assert_eq!(calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(counts), None);
    }
}
