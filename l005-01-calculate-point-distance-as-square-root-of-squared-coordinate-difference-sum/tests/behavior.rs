use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences;

#[test]
fn distance_extracts_root_of_squared_differences() {
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &[1.0, 2.0],
            &[4.0, 6.0]
        ),
        Ok(5.0)
    );
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &[4.0, 6.0],
            &[1.0, 2.0]
        ),
        Ok(5.0)
    );
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &[1.0, 2.0],
            &[1.0, 2.0]
        ),
        Ok(0.0)
    );
    assert!(
        (calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &[0.0, 0.0],
            &[1.0, 1.0]
        )
        .unwrap()
            - 2.0_f64.sqrt())
        .abs()
            < 1e-12
    );
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(&[], &[]),
        Ok(0.0)
    );
    assert!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &[1.0],
            &[]
        )
        .is_err()
    );
    assert!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &[],
            &[1.0]
        )
        .is_err()
    );
}
