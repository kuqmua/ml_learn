use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l005_01_calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

#[test]
#[ignore = "сначала вычисли расстояние вручную, затем запусти тест с --ignored"]
fn predict_dist_and_check_symmetry() {
    let expected_dist: Option<f64> = None; // Заполни: расстояние между [1, 2] и [4, 6]
    let expected_dist: f64 = expected_dist.expect("заполни ответ перед запуском теста");
    let first_point: [f64; 2] = [1.0, 2.0];
    let second_point: [f64; 2] = [4.0, 6.0];
    assert!(
        check_f64_eq_1e_minus_10(calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &second_point
        )
        .unwrap(), expected_dist)
    );
    assert_eq!(
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &second_point
        ),
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &second_point,
            &first_point
        )
    );
    assert_eq!(
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &first_point
        ),
        Ok(0.0)
    );
}
