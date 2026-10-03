use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs::calc_squared_point_dist_by_summing_squared_coord_diffs;

#[test]
fn known_dist_symmetry_translation_and_scaling() {
    let a = [-1.0, 2.0, 3.0];
    let b = [2.0, 6.0, 3.0];
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&a, &b),
        Ok(25.0)
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&b, &a),
        Ok(25.0)
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&a, &a),
        Ok(0.0)
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(
            &[9.0, 12.0, 13.0],
            &[12.0, 16.0, 13.0]
        ),
        Ok(25.0)
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(
            &[-2.0, 4.0, 6.0],
            &[4.0, 12.0, 6.0]
        ),
        Ok(100.0)
    );
}

#[test]
fn invalid_coords_are_rejected() {
    assert!(calc_squared_point_dist_by_summing_squared_coord_diffs(&[], &[]).is_err());
    assert!(calc_squared_point_dist_by_summing_squared_coord_diffs(&[f64::NAN], &[1.0]).is_err());
    assert!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&[f64::MAX], &[-f64::MAX]).is_err()
    );
}
