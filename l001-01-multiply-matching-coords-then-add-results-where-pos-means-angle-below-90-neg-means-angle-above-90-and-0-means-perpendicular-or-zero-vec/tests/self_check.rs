use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

#[test]
#[ignore = "заполни три вектора и запусти cargo test -p l001-01-multiply-matching-coords-then-add-results --test self_check -- --ignored"]
fn predict_signs_after_multiplying_matching_coords_then_adding() {
    let fixed_vec: [f64; 2] = [1.0, 2.0];
    let vec_with_neg_result: Option<[f64; 2]> = None; // Заполни: подбери вектор с отрицательным результатом
    let vec_with_neg_result: [f64; 2] =
        vec_with_neg_result.expect("заполни ответ перед запуском теста");
    let vec_with_zero_result: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let vec_with_zero_result: [f64; 2] =
        vec_with_zero_result.expect("заполни ответ перед запуском теста");
    let vec_with_pos_result: Option<[f64; 2]> = None; // Заполни: подбери вектор с положительным результатом
    let vec_with_pos_result: [f64; 2] =
        vec_with_pos_result.expect("заполни ответ перед запуском теста");

    assert!(
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&fixed_vec, &vec_with_neg_result)
            .unwrap()
            < 0.0
    );
    assert_eq!(
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&fixed_vec, &vec_with_zero_result)
            .unwrap(),
        0.0
    );
    assert!(vec_with_zero_result.iter().any(|&coord| coord != 0.0));
    assert!(
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&fixed_vec, &vec_with_pos_result)
            .unwrap()
            > 0.0
    );
}
