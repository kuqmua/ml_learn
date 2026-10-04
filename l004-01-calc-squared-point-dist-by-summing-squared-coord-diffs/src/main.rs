// Урок 004. Измеряем, насколько далеко друг от друга две точки, пока без извлечения корня.
// Вычитаем числа на одинаковых местах, умножаем каждую разницу саму на себя и складываем.
// Для [1, 2] и [4, 6] получаем 3×3 + 4×4 = 25.
// Ноль означает одну и ту же точку. Чем больше результат, тем дальше точки друг от друга.
// Чтобы получить обычное расстояние, из этого результата нужно взять квадратный корень.

use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs::calc_squared_point_dist_by_summing_squared_coord_diffs;

fn main() {
    let point1 = [1.0, 2.0];
    let point2 = [4.0, 6.0];

    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&point1, &point2,).unwrap(),
        25.0
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&point2, &point1),
        Ok(25.0)
    );
    assert_eq!(
        calc_squared_point_dist_by_summing_squared_coord_diffs(&point1, &point1),
        Ok(0.0)
    );
}

// Чему учит этот урок:
// Учимся считать квадрат расстояния между точками через разницы координат.
// Так можно сравнивать удалённость точек без извлечения корня; совпадающие точки дают ноль.
