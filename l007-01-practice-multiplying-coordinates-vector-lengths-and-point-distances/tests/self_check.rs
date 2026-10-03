use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coords_then_add_results_as_unnormalized_alignment_where_pos_means_angle_below_90_degrees_neg_means_angle_above_90_degrees_and_0_means_perpendicular_or_zero_vec;
use l002_01_calculate_sum_of_absolute_vector_coordinates::calc_sum_of_absolute_vec_coords_as_total_axis_aligned_length_where_0_means_zero_vec;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calc_vec_length_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer;
use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;
use l006_01_calculate_cos_of_angle_between_vectors::calc_cos_of_angle_between_vecs_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite;

#[test]
#[ignore = "подбери второй вектор и запусти тест с --ignored"]
fn combine_vec_properties() {
    let first_vec: [f64; 2] = [3.0, 4.0];
    let perpendicular_vec: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let perpendicular_vec: [f64; 2] =
        perpendicular_vec.expect("заполни ответ перед запуском теста");
    let expected_dist: Option<f64> = None; // Заполни: вычисли расстояние до выбранного вектора
    let expected_dist: f64 = expected_dist.expect("заполни ответ перед запуском теста");

    assert_eq!(
        multiply_matching_coords_then_add_results_as_unnormalized_alignment_where_pos_means_angle_below_90_degrees_neg_means_angle_above_90_degrees_and_0_means_perpendicular_or_zero_vec(&first_vec, &perpendicular_vec),
        Ok(0.0)
    );
    assert!(perpendicular_vec.iter().any(|&coord| coord != 0.0));
    assert_eq!(
        calc_sum_of_absolute_vec_coords_as_total_axis_aligned_length_where_0_means_zero_vec(
            &first_vec
        ),
        7.0
    );
    assert_eq!(
        calc_vec_length_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer(&first_vec),
        5.0
    );
    assert!(
        check_f64_eq_1e_minus_10(calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_vec,
            &perpendicular_vec
        )
        .unwrap(), expected_dist)
    );
    assert_eq!(
        calc_cos_of_angle_between_vecs_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&first_vec, &perpendicular_vec),
        Ok(0.0)
    );
}
