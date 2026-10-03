use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector;
use l002_01_calculate_sum_of_absolute_vector_coordinates::calculate_sum_of_absolute_vector_coordinates_as_total_axis_aligned_length_where_0_means_zero_vector;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer;
use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther;
use l006_01_calculate_cos_of_angle_between_vectors::calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite;

#[test]
#[ignore = "подбери второй вектор и запусти тест с --ignored"]
fn combine_vector_properties() {
    let first_vector: [f64; 2] = [3.0, 4.0];
    let perpendicular_vector: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let perpendicular_vector: [f64; 2] =
        perpendicular_vector.expect("заполни ответ перед запуском теста");
    let expected_distance: Option<f64> = None; // Заполни: вычисли расстояние до выбранного вектора
    let expected_distance: f64 = expected_distance.expect("заполни ответ перед запуском теста");

    assert_eq!(
        multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(&first_vector, &perpendicular_vector),
        Ok(0.0)
    );
    assert!(
        perpendicular_vector
            .iter()
            .any(|&coordinate| coordinate != 0.0)
    );
    assert_eq!(
        calculate_sum_of_absolute_vector_coordinates_as_total_axis_aligned_length_where_0_means_zero_vector(&first_vector),
        7.0
    );
    assert_eq!(
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer(&first_vector),
        5.0
    );
    assert!(
        check_f64_eq_1e_minus_10(calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
            &first_vector,
            &perpendicular_vector
        )
        .unwrap(), expected_distance)
    );
    assert_eq!(
        calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&first_vector, &perpendicular_vector),
        Ok(0.0)
    );
}
