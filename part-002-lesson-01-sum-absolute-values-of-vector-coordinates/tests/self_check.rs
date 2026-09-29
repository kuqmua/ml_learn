#[test]
#[ignore = "сначала вычисли ответы вручную, затем запусти тест с --ignored"]
fn predict_sum_absolute_values_of_vector_coordinates_after_scaling() {
    let expected_original: Option<f64> = None; // Заполни: L1-норма [-3, 4]
    let expected_original: f64 = expected_original.expect("заполни ответ перед запуском теста");
    let expected_scaled: Option<f64> = None; // Заполни: L1-норма [-6, 8]
    let expected_scaled: f64 = expected_scaled.expect("заполни ответ перед запуском теста");
    assert_eq!(
        part_002_lesson_01_sum_absolute_values_of_vector_coordinates::sum_absolute_values_of_vector_coordinates(&[
            -3.0, 4.0
        ]),
        expected_original
    );
    assert_eq!(
        part_002_lesson_01_sum_absolute_values_of_vector_coordinates::sum_absolute_values_of_vector_coordinates(&[
            -6.0, 8.0
        ]),
        expected_scaled
    );
    assert_eq!(expected_scaled, 2.0 * expected_original);
}
