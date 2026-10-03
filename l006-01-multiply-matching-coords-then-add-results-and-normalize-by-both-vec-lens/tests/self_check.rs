use l006_01_multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens;

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
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
            &[1.0, 0.0],
            &[2.0, 0.0]
        ),
        Ok(expected_direction_similarity_for_same_direction)
    );
    assert_eq!(
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
            &[1.0, 0.0],
            &[0.0, 1.0]
        ),
        Ok(expected_right_angle)
    );
    assert_eq!(
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
            &[1.0, 0.0],
            &[-1.0, 0.0]
        ),
        Ok(expected_direction_similarity_for_opposite_directions)
    );
    assert!(
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
            &[1.0, 0.0],
            &[0.0, 0.0]
        )
        .is_err()
    );
}
