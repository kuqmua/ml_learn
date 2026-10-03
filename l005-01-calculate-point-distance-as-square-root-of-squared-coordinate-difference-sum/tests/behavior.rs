use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther;

#[test]
fn distance_extracts_root_of_squared_differences() {
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[1.0, 2.0],
            &[4.0, 6.0]
        ),
        Ok(5.0)
    );
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[4.0, 6.0],
            &[1.0, 2.0]
        ),
        Ok(5.0)
    );
    assert_eq!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[1.0, 2.0],
            &[1.0, 2.0]
        ),
        Ok(0.0)
    );
    assert!(
        check_f64_eq_1e_minus_12(calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[0.0, 0.0],
            &[1.0, 1.0]
        )
        .unwrap(), 2.0_f64.sqrt())
    );
    assert!(
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[f64::NAN, 0.0],
            &[0.0, 0.0]
        )
        .is_err()
    );
}
