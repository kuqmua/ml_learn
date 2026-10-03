use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calc_text_context_vecs_by_adding_position_and_weighted_past_context;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли первую координату контекстного вектора для одного токена с id=0.
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    let actual = calc_text_context_vecs_by_adding_position_and_weighted_past_context(&[0])[0][0];
    assert!(check_f64_eq_1e_minus_12(actual, expected));
}
