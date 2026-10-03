use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_targets_and_scores_at_threshold;

#[test]
fn threshold_includes_equal_score_and_separates_four_outcomes() {
    assert_eq!(
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &[true, false, true, false],
            &[0.5, 0.8, 0.1, 0.2],
            0.5
        ),
        Ok(BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 1,
            false_poss_as_false_alarms_on_neg_cases: 1,
            true_negs_as_correctly_rejected_neg_cases: 1,
            false_negs_as_missed_pos_cases: 1
        })
    );
    assert_eq!(
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &[true, false],
            &[0.5, 0.5],
            0.5
        ),
        Ok(BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 1,
            false_poss_as_false_alarms_on_neg_cases: 1,
            true_negs_as_correctly_rejected_neg_cases: 0,
            false_negs_as_missed_pos_cases: 0
        })
    );
    assert_eq!(
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &[true, false],
            &[0.5, 0.5],
            0.6
        ),
        Ok(BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 0,
            false_poss_as_false_alarms_on_neg_cases: 0,
            true_negs_as_correctly_rejected_neg_cases: 1,
            false_negs_as_missed_pos_cases: 1
        })
    );
}

#[test]
fn empty_input_and_length_mismatch() {
    assert_eq!(
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(&[], &[], 0.5),
        Ok(BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases: 0,
            false_poss_as_false_alarms_on_neg_cases: 0,
            true_negs_as_correctly_rejected_neg_cases: 0,
            false_negs_as_missed_pos_cases: 0
        })
    );
    assert!(
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &[true],
            &[],
            0.5
        )
        .is_err()
    );
    assert!(
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(&[], &[0.2], 0.5)
            .is_err()
    );
}
