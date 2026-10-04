// Урок 005. Считаем расстояние между двумя точками по прямой.
// Берём сумму квадратов разниц из предыдущего урока и извлекаем квадратный корень.
// Например, для разниц 3 и 4 получаем корень из 25, то есть расстояние 5.
// Число координат у обеих точек должно совпадать; это проверяет тип массива.
// Бесконечные и неопределённые координаты, а также переполнение дают ошибку.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l005_01_calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs::calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs;

fn main() {
    let cases: [(&str, &[f64; 2], &[f64; 2], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    for (_description, point1, point2, expected) in cases {
        assert!(check_f64_eq_1e_minus_10(calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs(
                point1,
                point2,
            )
            .expect("не удалось вычислить расстояние: координаты должны быть конечными, а квадрат расстояния — помещаться в f64"), expected));
    }
}
