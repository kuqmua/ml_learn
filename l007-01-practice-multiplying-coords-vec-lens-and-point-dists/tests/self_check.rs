use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;
use l002_01_calc_sum_of_absolute_vec_coords::calc_sum_of_absolute_vec_coords;
use l003_01_calc_vec_len_as_square_root_of_sum_of_squared_coords::calc_vec_len_as_square_root_of_sum_of_squared_coords;
use l005_01_calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs::calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs;
use l006_01_multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens;

#[test]
#[ignore = "подбери второй вектор и запусти тест с --ignored"]
fn combine_vec_properties() {
    let vec1: [f64; 2] = [3.0, 4.0];
    let perpendicular_vec: Option<[f64; 2]> = None; // Заполни: подбери ненулевой перпендикулярный вектор
    let perpendicular_vec: [f64; 2] =
        perpendicular_vec.expect("заполни ответ перед запуском теста");
    let expected_dist: Option<f64> = None; // Заполни: вычисли расстояние до выбранного вектора
    let expected_dist: f64 = expected_dist.expect("заполни ответ перед запуском теста");

    assert_eq!(
        multiply_matching_coords_then_add_results(&vec1, &perpendicular_vec),
        Ok(0.0)
    );
    assert!(perpendicular_vec.iter().any(|&coord| coord != 0.0));
    assert_eq!(calc_sum_of_absolute_vec_coords(&vec1), 7.0);
    assert_eq!(
        calc_vec_len_as_square_root_of_sum_of_squared_coords(&vec1),
        5.0
    );
    assert!(check_f64_eq_1e_minus_10(
        calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs(&vec1, &perpendicular_vec)
            .unwrap(),
        expected_dist
    ));
    assert_eq!(
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
            &vec1,
            &perpendicular_vec
        ),
        Ok(0.0)
    );
}
