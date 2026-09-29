#[test]
#[ignore = "подбери второй вектор и запусти тест с --ignored"]
fn combine_vector_properties() {
    let first_vector = [3.0, 4.0];
    let perpendicular_vector: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let perpendicular_vector = perpendicular_vector.expect("заполни ответ перед запуском теста");
    let expected_distance: Option<f64> = None; // Заполни: вычисли расстояние до выбранного вектора
    let expected_distance = expected_distance.expect("заполни ответ перед запуском теста");

    assert_eq!(
        part_001_lesson_01_multiply_coordinates_and_add::multiply_matching_coordinates_then_add(
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
        part_002_lesson_01_sum_of_absolute_coordinate_values::sum_of_absolute_coordinates(
            &first_vector
        ),
        7.0
    );
    assert_eq!(
        part_003_lesson_01_euclidean_vector_length::euclidean_norm(&first_vector),
        5.0
    );
    assert!(
        (part_004_lesson_01_distance::euclidean_distance_between_points(
            &first_vector,
            &perpendicular_vector
        )
        .unwrap()
            - expected_distance)
            .abs()
            < 1e-10
    );
    assert_eq!(
        part_005_lesson_01_cosine_similarity::cosine_similarity(
            &first_vector,
            &perpendicular_vector
        ),
        Ok(0.0)
    );
}
