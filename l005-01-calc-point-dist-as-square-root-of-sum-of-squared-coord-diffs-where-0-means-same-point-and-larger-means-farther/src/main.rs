// Урок 005. Считаем расстояние между двумя точками по прямой.
// Берём сумму квадратов разниц из предыдущего урока и извлекаем квадратный корень.
// Например, для разниц 3 и 4 получаем корень из 25, то есть расстояние 5.
// Число координат у обеих точек должно совпадать; это проверяет тип массива.
// Бесконечные и неопределённые координаты, а также переполнение дают ошибку.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l005_01_calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

fn main() {
    let cases: [(&str, &[f64; 2], &[f64; 2], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    for (_description, first_point, second_point, expected) in cases {
        assert!(check_f64_eq_1e_minus_10(calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
                first_point,
                second_point,
            )
            .expect("не удалось вычислить расстояние: координаты должны быть конечными, а квадрат расстояния — помещаться в f64"), expected));
    }
    plot_dist_from_origin_for_changing_first_coord();
}

// Строим график по результатам урока.
fn plot_dist_from_origin_for_changing_first_coord() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Расстояние до начала координат",
        "x точки [x, 4]",
        "расстояние",
        &[lesson_visualization::Series {
            name: "расстояние",

            points: &(-50..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (
                horizontal_value,
                calc_point_dist_as_square_root_of_sum_of_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
                    &[0.0, 0.0],
                    &[horizontal_value, 4.0],
                )
                .unwrap(),
            )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
