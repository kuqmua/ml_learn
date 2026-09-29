use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences as distance;

#[test]
fn distance_extracts_root_of_squared_differences() {
    assert_eq!(distance(&[1.0, 2.0], &[4.0, 6.0]), Ok(5.0));
    assert_eq!(distance(&[4.0, 6.0], &[1.0, 2.0]), Ok(5.0));
    assert_eq!(distance(&[1.0, 2.0], &[1.0, 2.0]), Ok(0.0));
    assert!((distance(&[0.0, 0.0], &[1.0, 1.0]).unwrap() - 2.0_f64.sqrt()).abs() < 1e-12);
    assert_eq!(distance(&[], &[]), Ok(0.0));
    assert!(distance(&[1.0], &[]).is_err());
    assert!(distance(&[], &[1.0]).is_err());
}
