use l006_01_calculate_cos_of_angle_between_vectors::calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite;

#[test]
#[ignore = "сначала вычисли сходство вручную, затем запусти тест с --ignored"]
fn predict_three_directions() {
    let expected_direction_similarity_for_same_direction: Option<f64> = None; // Заполни: сходство [1, 0] и [2, 0]
    let expected_direction_similarity_for_same_direction: f64 =
        expected_direction_similarity_for_same_direction
            .expect("заполни ответ перед запуском теста");
    let expected_right_angle: Option<f64> = None; // Заполни: сходство [1, 0] и [0, 1]
    let expected_right_angle: f64 =
        expected_right_angle.expect("заполни ответ перед запуском теста");
    let expected_direction_similarity_for_opposite_directions: Option<f64> = None; // Заполни: сходство [1, 0] и [-1, 0]
    let expected_direction_similarity_for_opposite_directions: f64 =
        expected_direction_similarity_for_opposite_directions
            .expect("заполни ответ перед запуском теста");
    assert_eq!(
        calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[2.0, 0.0]),
        Ok(expected_direction_similarity_for_same_direction)
    );
    assert_eq!(
        calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[0.0, 1.0]),
        Ok(expected_right_angle)
    );
    assert_eq!(
        calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[-1.0, 0.0]),
        Ok(expected_direction_similarity_for_opposite_directions)
    );
    assert!(calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&[1.0, 0.0], &[0.0, 0.0]).is_err());
}
