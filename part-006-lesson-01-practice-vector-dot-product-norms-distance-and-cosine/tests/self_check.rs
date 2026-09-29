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
        part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::multiply_matching_coordinates_of_two_vectors_then_add(
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
        part_002_lesson_01_sum_absolute_values_of_vector_coordinates::sum_absolute_values_of_vector_coordinates(
            &first_vector
        ),
        7.0
    );
    assert_eq!(
        part_003_lesson_01_calculate_euclidean_length_of_one_vector::euclidean_norm_of_vector(
            &first_vector
        ),
        5.0
    );
    assert!(
        (part_004_lesson_01_euclidean_distance_between_two_points::euclidean_distance_between_two_points(
            &first_vector,
            &perpendicular_vector
        )
        .unwrap()
            - expected_distance)
            .abs()
            < 1e-10
    );
    assert_eq!(
        part_005_lesson_01_cosine_similarity_between_two_vectors::cosine_similarity_between_two_vectors(
            &first_vector,
            &perpendicular_vector
        ),
        Ok(0.0)
    );
}
