use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts as Counts;
use l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold as operation;

#[test]
fn threshold_includes_equal_score_and_separates_four_outcomes() {
    assert_eq!(
        operation(&[true, false, true, false], &[0.5, 0.8, 0.1, 0.2], 0.5),
        Ok(Counts {
            true_positives: 1,
            false_positives: 1,
            true_negatives: 1,
            false_negatives: 1
        })
    );
    assert_eq!(
        operation(&[true, false], &[0.5, 0.5], 0.5),
        Ok(Counts {
            true_positives: 1,
            false_positives: 1,
            true_negatives: 0,
            false_negatives: 0
        })
    );
    assert_eq!(
        operation(&[true, false], &[0.5, 0.5], 0.6),
        Ok(Counts {
            true_positives: 0,
            false_positives: 0,
            true_negatives: 1,
            false_negatives: 1
        })
    );
}

#[test]
fn empty_input_and_length_mismatch() {
    assert_eq!(
        operation(&[], &[], 0.5),
        Ok(Counts {
            true_positives: 0,
            false_positives: 0,
            true_negatives: 0,
            false_negatives: 0
        })
    );
    assert!(operation(&[true], &[], 0.5).is_err());
    assert!(operation(&[], &[0.2], 0.5).is_err());
}
