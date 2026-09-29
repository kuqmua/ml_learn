#[test]
#[ignore = "сначала вычисли ответы вручную, затем запусти тест с --ignored"]
fn predict_vector_length_after_doubling_coordinates() {
    let expected_original: Option<f64> = None; // Заполни: длина вектора [3, 4]
    let expected_original: f64 = expected_original.expect("заполни ответ перед запуском теста");
    let expected_scaled: Option<f64> = None; // Заполни: длина вектора [6, 8]
    let expected_scaled: f64 = expected_scaled.expect("заполни ответ перед запуском теста");
    assert!(
        (part_003_lesson_01_square_root_of_sum_of_squared_vector_coordinates::square_root_of_sum_of_squared_coordinates(&[
            3.0, 4.0
        ]) - expected_original)
            .abs()
            < 1e-10
    );
    assert!(
        (part_003_lesson_01_square_root_of_sum_of_squared_vector_coordinates::square_root_of_sum_of_squared_coordinates(&[
            6.0, 8.0
        ]) - expected_scaled)
            .abs()
            < 1e-10
    );
    assert!((expected_scaled - 2.0 * expected_original).abs() < 1e-10);
}
