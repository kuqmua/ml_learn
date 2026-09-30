use l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Сколько TP при метках [true, false], оценках [0.5, 0.8] и пороге 0.5?
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    let actual = count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(
        &[true, false],
        &[0.5, 0.8],
        0.5,
    )
    .unwrap()
    .true_positives as f64;
    assert!((actual - expected).abs() < 1e-12);
}
