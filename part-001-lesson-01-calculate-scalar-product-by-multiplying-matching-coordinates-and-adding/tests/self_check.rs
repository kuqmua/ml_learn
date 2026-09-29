#[test]
#[ignore = "заполни три вектора и запусти cargo test -p part-001-lesson-01-calculate-scalar-product-by-multiplying-matching-coordinates-and-adding --test self_check -- --ignored"]
fn predict_signs_of_summed_matching_coordinate_products() {
    let fixed_vector: [f64; 2] = [1.0, 2.0];
    let vector_with_negative_result: Option<[f64; 2]> = None; // Заполни: подбери вектор с отрицательным результатом
    let vector_with_negative_result: [f64; 2] =
        vector_with_negative_result.expect("заполни ответ перед запуском теста");
    let vector_with_zero_result: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let vector_with_zero_result: [f64; 2] =
        vector_with_zero_result.expect("заполни ответ перед запуском теста");
    let vector_with_positive_result: Option<[f64; 2]> = None; // Заполни: подбери вектор с положительным результатом
    let vector_with_positive_result: [f64; 2] =
        vector_with_positive_result.expect("заполни ответ перед запуском теста");

    let negative_result: f64 =
        part_001_lesson_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
            &fixed_vector,
            &vector_with_negative_result,
        )
        .unwrap();
    let zero_result: f64 =
        part_001_lesson_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
            &fixed_vector,
            &vector_with_zero_result,
        )
        .unwrap();
    let positive_result: f64 =
        part_001_lesson_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
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
