use part_005_lesson_01_cosine_similarity::cosine_similarity;

#[test]
#[ignore = "сначала вычисли сходство вручную, затем запусти тест с --ignored"]
fn predict_three_directions() {
    let expected_same_direction: Option<f64> = None; // Заполни: сходство [1, 0] и [2, 0]
    let expected_same_direction =
        expected_same_direction.expect("заполни ответ перед запуском теста");
    let expected_right_angle: Option<f64> = None; // Заполни: сходство [1, 0] и [0, 1]
    let expected_right_angle = expected_right_angle.expect("заполни ответ перед запуском теста");
    let expected_opposite_direction: Option<f64> = None; // Заполни: сходство [1, 0] и [-1, 0]
    let expected_opposite_direction =
        expected_opposite_direction.expect("заполни ответ перед запуском теста");
    assert_eq!(
        cosine_similarity(&[1.0, 0.0], &[2.0, 0.0]),
        Ok(expected_same_direction)
    );
    assert_eq!(
        cosine_similarity(&[1.0, 0.0], &[0.0, 1.0]),
        Ok(expected_right_angle)
    );
    assert_eq!(
        cosine_similarity(&[1.0, 0.0], &[-1.0, 0.0]),
        Ok(expected_opposite_direction)
    );
    assert!(cosine_similarity(&[1.0, 0.0], &[0.0, 0.0]).is_err());
}
