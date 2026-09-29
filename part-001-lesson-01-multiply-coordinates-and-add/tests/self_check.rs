#[test]
#[ignore = "заполни три вектора и запусти cargo test -p part-001-lesson-01-multiply-coordinates-and-add --test self_check -- --ignored"]
fn predict_signs_of_scalar_products() {
    let fixed_vector = [1.0, 2.0];
    let vector_with_negative_result: Option<[f64; 2]> = None; // Заполни: подбери вектор с отрицательным результатом
    let vector_with_negative_result =
        vector_with_negative_result.expect("заполни ответ перед запуском теста");
    let vector_with_zero_result: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let vector_with_zero_result =
        vector_with_zero_result.expect("заполни ответ перед запуском теста");
    let vector_with_positive_result: Option<[f64; 2]> = None; // Заполни: подбери вектор с положительным результатом
    let vector_with_positive_result =
        vector_with_positive_result.expect("заполни ответ перед запуском теста");

    let negative_result =
        part_001_lesson_01_multiply_coordinates_and_add::multiply_matching_coordinates_then_add(
            &fixed_vector,
            &vector_with_negative_result,
        )
        .unwrap();
    let zero_result =
        part_001_lesson_01_multiply_coordinates_and_add::multiply_matching_coordinates_then_add(
            &fixed_vector,
            &vector_with_zero_result,
        )
        .unwrap();
    let positive_result =
        part_001_lesson_01_multiply_coordinates_and_add::multiply_matching_coordinates_then_add(
            &fixed_vector,
            &vector_with_positive_result,
        )
        .unwrap();
    assert!(negative_result < 0.0);
    assert_eq!(zero_result, 0.0);
    assert!(
        vector_with_zero_result
            .iter()
            .any(|&coordinate| coordinate != 0.0)
    );
    assert!(positive_result > 0.0);
}
