// Урок 007. Сравниваем несколько свойств одной пары списков чисел.
// Отдельно считаем сумму величин координат без знаков, длину стрелки, расстояние
// между точками и сходство направлений. Это ответы на разные вопросы.
// Например, одинаковое направление не означает одинаковую длину.
// Проверяем также противоположные направления, угол 90° и нулевую стрелку.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;
use l002_01_calc_sum_of_absolute_vec_coords::calc_sum_of_absolute_vec_coords;
use l003_01_calc_vec_len_as_square_root_of_sum_of_squared_coords::calc_vec_len_as_square_root_of_sum_of_squared_coords;
use l005_01_calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs::calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs;
use l006_01_multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens;

fn main() {
    let vec1: [f64; 2] = [3.0, 4.0];
    assert_eq!(calc_sum_of_absolute_vec_coords(&vec1), 7.0);
    assert_eq!(
        calc_vec_len_as_square_root_of_sum_of_squared_coords(&vec1),
        5.0
    );

    let cases: [(&str, &[f64; 2], f64, Option<f64>); 4] = [
        ("тот же вектор", &[3.0, 4.0], 25.0, Some(1.0)),
        ("перпендикулярный", &[-4.0, 3.0], 0.0, Some(0.0)),
        ("противоположный", &[-3.0, -4.0], -25.0, Some(-1.0)),
        ("нулевой без направления", &[0.0, 0.0], 0.0, None),
    ];
    for (_description, vec2, expected_sum, expected_direction_similarity) in cases {
        let direction_similarity: Option<f64> =
            multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(&vec1, vec2)
                .ok();
        assert_eq!(
            multiply_matching_coords_then_add_results(&vec1, vec2)
                .expect("ожидались векторы с одинаковым числом координат"),
            expected_sum
        );
        if let (Some(actual), Some(expected)) =
            (direction_similarity, expected_direction_similarity)
        {
            assert!(check_f64_eq_1e_minus_10(actual, expected));
        } else {
            assert_eq!(direction_similarity, expected_direction_similarity);
        }
        assert!(check_f64_eq_1e_minus_10(
            calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs(&vec1, vec2,).unwrap(),
            calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs(vec2, &vec1,).unwrap()
        ));
    }

    let too_short: [f64; 1] = [1.0];
    let _: &str = multiply_matching_coords_then_add_results(&vec1, &too_short)
        .expect_err("векторы разной длины нужно отклонить");
}

// Чему учит этот урок:
// Учимся выбирать вычисление под вопрос: длина, расстояние и сходство направлений измеряют разное.
// Соединяем функции предыдущих уроков и проверяем особые случаи, включая нулевой вектор и разные
// размеры.
