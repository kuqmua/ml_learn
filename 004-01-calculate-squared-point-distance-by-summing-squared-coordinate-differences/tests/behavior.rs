use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences as operation;

#[test]
fn known_distance_symmetry_translation_and_scaling() {
    let a = [-1.0, 2.0, 3.0];
    let b = [2.0, 6.0, 3.0];
    assert_eq!(operation(&a, &b), Ok(25.0));
    assert_eq!(operation(&b, &a), Ok(25.0));
    assert_eq!(operation(&a, &a), Ok(0.0));
    assert_eq!(operation(&[9.0, 12.0, 13.0], &[12.0, 16.0, 13.0]), Ok(25.0));
    assert_eq!(operation(&[-2.0, 4.0, 6.0], &[4.0, 12.0, 6.0]), Ok(100.0));
}

#[test]
fn empty_vectors_and_mismatched_dimensions() {
    assert_eq!(operation(&[], &[]), Ok(0.0));
    assert!(operation(&[1.0], &[]).is_err());
    assert!(operation(&[], &[1.0]).is_err());
}
