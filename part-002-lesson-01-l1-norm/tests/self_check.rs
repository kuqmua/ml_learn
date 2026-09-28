use part_002_lesson_01_l1_norm::sum_of_absolute_coordinates;

#[test]
#[ignore = "сначала вычисли ответы вручную, затем запусти тест с --ignored"]
fn predict_sum_of_absolute_coordinates_after_scaling() {
    let expected_original: Option<f64> = None; // Заполни: L1-норма [-3, 4]
    let expected_original = expected_original.expect("заполни ответ перед запуском теста");
    let expected_scaled: Option<f64> = None; // Заполни: L1-норма [-6, 8]
    let expected_scaled = expected_scaled.expect("заполни ответ перед запуском теста");
    assert_eq!(sum_of_absolute_coordinates(&[-3.0, 4.0]), expected_original);
    assert_eq!(sum_of_absolute_coordinates(&[-6.0, 8.0]), expected_scaled);
    assert_eq!(expected_scaled, 2.0 * expected_original);
}
