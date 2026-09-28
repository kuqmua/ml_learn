use lesson_001::multiply_matching_coordinates_then_add;
use lesson_002::sum_of_absolute_coordinates;
use lesson_003::euclidean_norm;
use lesson_004::distance;
use lesson_005::cosine_similarity;

#[test]
#[ignore = "подбери второй вектор и запусти тест с --ignored"]
fn combine_vector_properties() {
    let first_vector = [3.0, 4.0];
    let perpendicular_vector: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let perpendicular_vector = perpendicular_vector.expect("заполни ответ перед запуском теста");
    let expected_distance: Option<f64> = None; // Заполни: вычисли расстояние до выбранного вектора
    let expected_distance = expected_distance.expect("заполни ответ перед запуском теста");

    assert_eq!(
        multiply_matching_coordinates_then_add(&first_vector, &perpendicular_vector),
        Ok(0.0)
    );
    assert!(
        perpendicular_vector
            .iter()
            .any(|&coordinate| coordinate != 0.0)
    );
    assert_eq!(sum_of_absolute_coordinates(&first_vector), 7.0);
    assert_eq!(euclidean_norm(&first_vector), 5.0);
    assert!(
        (distance(&first_vector, &perpendicular_vector).unwrap() - expected_distance).abs() < 1e-10
    );
    assert_eq!(
        cosine_similarity(&first_vector, &perpendicular_vector),
        Ok(0.0)
    );
}
