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
        part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
            &first_vector,
            &perpendicular_vector
        ),
        Ok(0.0)
    );
    assert!(
        perpendicular_vector
            .iter()
            .any(|&coordinate| coordinate != 0.0)
    );
    assert_eq!(
        part_002_lesson_01_sum_absolute_values_of_vector_coordinates::calculate_l1_vector_norm_by_summing_absolute_coordinates(
            &first_vector
        ),
        7.0
    );
    assert_eq!(
        part_003_lesson_01_square_root_of_sum_of_squared_vector_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(
            &first_vector
        ),
        5.0
    );
    assert!(
        (part_004_lesson_01_square_root_of_sum_of_squared_coordinate_differences::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &first_vector,
            &perpendicular_vector
        )
        .unwrap()
            - expected_distance)
            .abs()
            < 1e-10
    );
    assert_eq!(
        part_005_lesson_01_sum_coordinate_products_and_divide_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
            &first_vector,
            &perpendicular_vector
        ),
        Ok(0.0)
    );
}
