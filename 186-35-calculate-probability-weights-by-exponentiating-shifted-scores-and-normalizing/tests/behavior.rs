use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum as operation;

#[test]
fn known_probabilities_are_positive_and_sum_to_one() {
    let weights = operation(&[0.0, 1.0, 2.0]);
    let expected = [0.09003057317038046, 0.24472847105479764, 0.6652409557748218];
    assert_eq!(weights.len(), expected.len());
    for (actual, expected) in weights.iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-12);
        assert!(*actual > 0.0);
    }
    assert!((weights.iter().sum::<f64>() - 1.0).abs() < 1e-12);
}

#[test]
fn common_shift_does_not_change_weights_or_overflow() {
    let expected = operation(&[0.0, 1.0, 2.0]);
    for scores in [[1000.0, 1001.0, 1002.0], [-1000.0, -999.0, -998.0]] {
        let actual = operation(&scores);
        assert_eq!(actual, expected);
        assert!(actual.iter().all(|value| value.is_finite()));
    }
}

#[test]
fn empty_single_equal_and_extreme_scores() {
    assert!(operation(&[]).is_empty());
    assert_eq!(operation(&[42.0]), vec![1.0]);
    assert_eq!(operation(&[3.0, 3.0, 3.0, 3.0]), vec![0.25; 4]);
    assert_eq!(operation(&[-1000.0, 1000.0]), vec![0.0, 1.0]);
}
