use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l005_01_calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

#[test]
fn dist_extracts_root_of_squared_diffs() {
    assert_eq!(
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &[1.0, 2.0],
            &[4.0, 6.0]
        ),
        Ok(5.0)
    );
    assert_eq!(
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &[4.0, 6.0],
            &[1.0, 2.0]
        ),
        Ok(5.0)
    );
    assert_eq!(
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &[1.0, 2.0],
            &[1.0, 2.0]
        ),
        Ok(0.0)
    );
    assert!(
        check_f64_eq_1e_minus_12(calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &[0.0, 0.0],
            &[1.0, 1.0]
        )
        .unwrap(), 2.0_f64.sqrt())
    );
    assert!(
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &[f64::NAN, 0.0],
            &[0.0, 0.0]
        )
        .is_err()
    );
}
