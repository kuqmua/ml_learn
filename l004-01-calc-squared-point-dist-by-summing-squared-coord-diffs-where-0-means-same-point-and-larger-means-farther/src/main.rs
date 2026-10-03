// Урок 004. Измеряем, насколько далеко друг от друга две точки, пока без извлечения корня.
// Вычитаем числа на одинаковых местах, умножаем каждую разницу саму на себя и складываем.
// Для [1, 2] и [4, 6] получаем 3×3 + 4×4 = 25.
// Ноль означает одну и ту же точку. Чем больше результат, тем дальше точки друг от друга.
// Чтобы получить обычное расстояние, из этого результата нужно взять квадратный корень.

use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

fn main() {
    let first_point = [1.0, 2.0];
    let second_point = [4.0, 6.0];

    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &second_point,
        )
        .unwrap(),
        25.0
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &second_point,
            &first_point
        ),
        Ok(25.0)
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_point,
            &first_point
        ),
        Ok(0.0)
    );
}
