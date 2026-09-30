#[test]
#[ignore = "сначала вычисли ответ вручную, затем запусти с --ignored"]
fn predict_result_before_running() {
    // Вычисли квадрат расстояния между [1, 2] и [4, 6].
    let expected: Option<f64> = None;
    let expected = expected.expect("впиши ответ перед запуском");
    let actual = l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&[1.0, 2.0], &[4.0, 6.0]).unwrap();
    assert!((actual - expected).abs() < 1e-12);
}
