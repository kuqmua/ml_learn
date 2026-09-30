#[test]
fn known_distance_symmetry_translation_and_scaling() {
    let a = [-1.0, 2.0, 3.0];
    let b = [2.0, 6.0, 3.0];
    assert_eq!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&a, &b), Ok(25.0));
    assert_eq!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&b, &a), Ok(25.0));
    assert_eq!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&a, &a), Ok(0.0));
    assert_eq!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&[9.0, 12.0, 13.0], &[12.0, 16.0, 13.0]), Ok(25.0));
    assert_eq!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&[-2.0, 4.0, 6.0], &[4.0, 12.0, 6.0]), Ok(100.0));
}

#[test]
fn empty_vectors_and_mismatched_dimensions() {
    assert_eq!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&[], &[]), Ok(0.0));
    assert!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&[1.0], &[]).is_err());
    assert!(l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&[], &[1.0]).is_err());
}
