// Урок 35.2. Устойчивый softmax перед расчётом причинного внимания.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares;

fn main() {
    let scores = [1000.0, 1001.0, 1002.0];
    let probabilities =
        calculate_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
            &scores,
        );

    assert!(check_f64_eq_1e_minus_12(
        probabilities.iter().sum::<f64>(),
        1.0
    ));

    assert_eq!(
        probabilities,
        calculate_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
            &[0.0, 1.0, 2.0],
        )
    );
}
