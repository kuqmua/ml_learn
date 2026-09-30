use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;
use l002_01_calculate_sum_of_absolute_vector_coordinates::calculate_sum_of_absolute_vector_coordinates;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates;
use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences;
use l006_01_calculate_direction_similarity_by_multiplying_coordinates_then_dividing_sum_by_lengths::calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths;

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
        multiply_matching_coordinates_then_add_results(&first_vector, &perpendicular_vector),
        Ok(0.0)
    );
    assert!(
        perpendicular_vector
            .iter()
            .any(|&coordinate| coordinate != 0.0)
    );
    assert_eq!(
        calculate_sum_of_absolute_vector_coordinates(&first_vector),
        7.0
    );
    assert_eq!(
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(&first_vector),
        5.0
    );
    assert!(
        (calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &first_vector,
            &perpendicular_vector
        )
        .unwrap()
            - expected_distance)
            .abs()
            < 1e-10
    );
    assert_eq!(
        calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths(
            &first_vector,
            &perpendicular_vector
        ),
        Ok(0.0)
    );
}
