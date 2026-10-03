use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_weight_for_equal_scores() {
    // Каков вес каждой из четырёх одинаковых оценок?
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    for actual in
        calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
            &[1000.0; 4],
        )
    {
        assert!(check_f64_eq_1e_minus_12(actual, expected));
    }
}
