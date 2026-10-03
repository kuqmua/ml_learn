use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther;

#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли квадрат расстояния между [1, 2] и [4, 6].
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    assert!(
        check_f64_eq_1e_minus_12(calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[1.0, 2.0],
            &[4.0, 6.0],
        )
        .unwrap(), expected)
    );
}
