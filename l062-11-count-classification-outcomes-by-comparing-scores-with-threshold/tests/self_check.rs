use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_targets_and_scores_at_threshold;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Сколько TP при метках [true, false], оценках [0.5, 0.8] и пороге 0.5?
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    let actual = count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
        &[true, false],
        &[0.5, 0.8],
        0.5,
    )
    .unwrap()
    .true_positives_as_correctly_detected_positive_cases as f64;
    assert!(check_f64_eq_1e_minus_12(actual, expected));
}
