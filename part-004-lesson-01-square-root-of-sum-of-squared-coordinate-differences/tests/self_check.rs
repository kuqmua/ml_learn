#[test]
#[ignore = "сначала вычисли расстояние вручную, затем запусти тест с --ignored"]
fn predict_distance_and_check_symmetry() {
    let expected_distance: Option<f64> = None; // Заполни: расстояние между [1, 2] и [4, 6]
    let expected_distance: f64 = expected_distance.expect("заполни ответ перед запуском теста");
    let first_point: [f64; 2] = [1.0, 2.0];
    let second_point: [f64; 2] = [4.0, 6.0];
    assert!(
        (part_004_lesson_01_square_root_of_sum_of_squared_coordinate_differences::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &first_point,
            &second_point
        )
        .unwrap()
            - expected_distance)
            .abs()
            < 1e-10
    );
    assert_eq!(
        part_004_lesson_01_square_root_of_sum_of_squared_coordinate_differences::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &first_point,
            &second_point
        ),
        part_004_lesson_01_square_root_of_sum_of_squared_coordinate_differences::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &second_point,
            &first_point
        )
    );
    assert_eq!(
        part_004_lesson_01_square_root_of_sum_of_squared_coordinate_differences::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &first_point,
            &first_point
        ),
        Ok(0.0)
    );
}
