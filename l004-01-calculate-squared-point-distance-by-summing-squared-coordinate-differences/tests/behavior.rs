use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther;

#[test]
fn known_distance_symmetry_translation_and_scaling() {
    let a = [-1.0, 2.0, 3.0];
    let b = [2.0, 6.0, 3.0];
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(&a, &b),
        Ok(25.0)
    );
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(&b, &a),
        Ok(25.0)
    );
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(&a, &a),
        Ok(0.0)
    );
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[9.0, 12.0, 13.0],
            &[12.0, 16.0, 13.0]
        ),
        Ok(25.0)
    );
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[-2.0, 4.0, 6.0],
            &[4.0, 12.0, 6.0]
        ),
        Ok(100.0)
    );
}

#[test]
fn invalid_coordinates_are_rejected() {
    assert!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(&[], &[])
            .is_err()
    );
    assert!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[f64::NAN],
            &[1.0]
        )
        .is_err()
    );
    assert!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &[f64::MAX],
            &[-f64::MAX]
        )
        .is_err()
    );
}
