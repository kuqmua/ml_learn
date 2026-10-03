// Квадрат расстояния: сложение квадратов разностей координат.

use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

fn main() {
    let first_point = [1.0, 2.0];
    let second_point = [4.0, 6.0];

    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &second_point,
        )
        .unwrap(),
        25.0
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &second_point,
            &first_point
        ),
        Ok(25.0)
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &first_point
        ),
        Ok(0.0)
    );
}
