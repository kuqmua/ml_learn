use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_weight_for_equal_scores() {
    // Каков вес каждой из четырёх одинаковых оценок?
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    for actual in
        calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &[1000.0; 4],
        )
    {
        assert!((actual - expected).abs() < 1e-12);
    }
}
