#[test]
#[ignore = "сначала вычисли расстояние вручную, затем запусти тест с --ignored"]
fn predict_distance_and_check_symmetry() {
    let expected_distance: Option<f64> = None; // Заполни: расстояние между [1, 2] и [4, 6]
    let expected_distance = expected_distance.expect("заполни ответ перед запуском теста");
    let first_point = [1.0, 2.0];
    let second_point = [4.0, 6.0];
    assert!(
        (part_004_lesson_01_distance::euclidean_distance_between_points(
            &first_point,
            &second_point
        )
        .unwrap()
            - expected_distance)
            .abs()
            < 1e-10
    );
    assert_eq!(
        part_004_lesson_01_distance::euclidean_distance_between_points(&first_point, &second_point),
        part_004_lesson_01_distance::euclidean_distance_between_points(&second_point, &first_point)
    );
    assert_eq!(
        part_004_lesson_01_distance::euclidean_distance_between_points(&first_point, &first_point),
        Ok(0.0)
    );
}
