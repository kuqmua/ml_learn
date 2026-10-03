use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs::calc_squared_point_dist_by_summing_squared_coord_diffs;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли квадрат расстояния между [1, 2] и [4, 6].
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    assert!(check_f64_eq_1e_minus_12(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&[1.0, 2.0], &[4.0, 6.0],).unwrap(),
        expected
    ));
}
