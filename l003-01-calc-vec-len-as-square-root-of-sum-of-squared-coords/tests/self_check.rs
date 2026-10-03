use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l003_01_calc_vec_len_as_square_root_of_sum_of_squared_coords::calc_vec_len_as_square_root_of_sum_of_squared_coords;

#[test]
#[ignore = "сначала вычисли ответы вручную, затем запусти тест с --ignored"]
fn predict_vec_len_after_doubling_coords() {
    let expected_original: Option<f64> = None; // Заполни: длина вектора [3, 4]
    let expected_original: f64 = expected_original.expect("заполни ответ перед запуском теста");
    let expected_scaled: Option<f64> = None; // Заполни: длина вектора [6, 8]
    let expected_scaled: f64 = expected_scaled.expect("заполни ответ перед запуском теста");
    assert!(check_f64_eq_1e_minus_10(
        calc_vec_len_as_square_root_of_sum_of_squared_coords(&[3.0, 4.0]),
        expected_original
    ));
    assert!(check_f64_eq_1e_minus_10(
        calc_vec_len_as_square_root_of_sum_of_squared_coords(&[6.0, 8.0]),
        expected_scaled
    ));
    assert!(check_f64_eq_1e_minus_10(
        expected_scaled,
        2.0 * expected_original
    ));
}
