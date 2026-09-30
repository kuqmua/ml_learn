use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;

#[test]
#[ignore = "заполни три вектора и запусти cargo test -p l001-01-multiply-matching-coordinates-then-add-results --test self_check -- --ignored"]
fn predict_signs_after_multiplying_matching_coordinates_then_adding() {
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
        multiply_matching_coordinates_then_add_results(&fixed_vector, &vector_with_negative_result)
            .unwrap();
    let zero_result: f64 =
        multiply_matching_coordinates_then_add_results(&fixed_vector, &vector_with_zero_result)
            .unwrap();
    let positive_result: f64 =
        multiply_matching_coordinates_then_add_results(&fixed_vector, &vector_with_positive_result)
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
