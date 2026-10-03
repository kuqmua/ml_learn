use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther;

#[test]
#[ignore = "сначала вычисли расстояние вручную, затем запусти тест с --ignored"]
fn predict_distance_and_check_symmetry() {
    let expected_distance: Option<f64> = None; // Заполни: расстояние между [1, 2] и [4, 6]
    let expected_distance: f64 = expected_distance.expect("заполни ответ перед запуском теста");
    let first_point: [f64; 2] = [1.0, 2.0];
    let second_point: [f64; 2] = [4.0, 6.0];
    assert!(
        (calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &second_point
        )
        .unwrap()
            - expected_distance)
            .abs()
            < 1e-10
    );
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &second_point
        ),
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &second_point,
            &first_point
        )
    );
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &first_point
        ),
        Ok(0.0)
    );
}
