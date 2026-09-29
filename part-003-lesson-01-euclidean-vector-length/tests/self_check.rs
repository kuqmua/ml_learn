#[test]
#[ignore = "сначала вычисли ответы вручную, затем запусти тест с --ignored"]
fn predict_euclidean_norm_after_scaling() {
    let expected_original: Option<f64> = None; // Заполни: евклидова норма [3, 4]
    let expected_original = expected_original.expect("заполни ответ перед запуском теста");
    let expected_scaled: Option<f64> = None; // Заполни: евклидова норма [6, 8]
    let expected_scaled = expected_scaled.expect("заполни ответ перед запуском теста");
    assert!(
        (part_003_lesson_01_euclidean_vector_length::euclidean_norm(&[3.0, 4.0])
            - expected_original)
            .abs()
            < 1e-10
    );
    assert!(
        (part_003_lesson_01_euclidean_vector_length::euclidean_norm(&[6.0, 8.0]) - expected_scaled)
            .abs()
            < 1e-10
    );
    assert!((expected_scaled - 2.0 * expected_original).abs() < 1e-10);
}
