use l006_01_calculate_cos_of_angle_between_vectors::calculate_cos_of_angle_between_vectors;

#[test]
#[ignore = "сначала вычисли сходство вручную, затем запусти тест с --ignored"]
fn predict_three_directions() {
    let expected_same_direction_cos: Option<f64> = None; // Заполни: сходство [1, 0] и [2, 0]
    let expected_same_direction_cos: f64 =
        expected_same_direction_cos.expect("заполни ответ перед запуском теста");
    let expected_right_angle: Option<f64> = None; // Заполни: сходство [1, 0] и [0, 1]
    let expected_right_angle: f64 =
        expected_right_angle.expect("заполни ответ перед запуском теста");
    let expected_opposite_direction_cos: Option<f64> = None; // Заполни: сходство [1, 0] и [-1, 0]
    let expected_opposite_direction_cos: f64 =
        expected_opposite_direction_cos.expect("заполни ответ перед запуском теста");
    assert_eq!(
        calculate_cos_of_angle_between_vectors(&[1.0, 0.0], &[2.0, 0.0]),
        Ok(expected_same_direction_cos)
    );
    assert_eq!(
        calculate_cos_of_angle_between_vectors(&[1.0, 0.0], &[0.0, 1.0]),
        Ok(expected_right_angle)
    );
    assert_eq!(
        calculate_cos_of_angle_between_vectors(&[1.0, 0.0], &[-1.0, 0.0]),
        Ok(expected_opposite_direction_cos)
    );
    assert!(calculate_cos_of_angle_between_vectors(&[1.0, 0.0], &[0.0, 0.0]).is_err());
}
