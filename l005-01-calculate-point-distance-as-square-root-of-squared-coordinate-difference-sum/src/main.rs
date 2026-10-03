// Урок 01.4. Расстояние между точками: квадратный корень из суммы квадратов разностей координат.
// Связь с принятой терминологией: Евклидово расстояние между точками.
// Зачем здесь эта тема: Квадрат расстояния из предыдущей части переводим в обычное расстояние.
// Почему код устроен так: Вызываем предыдущую часть и извлекаем корень стандартным sqrt.
//   Одинаковую размерность точек задаёт тип массива в сигнатуре функции.
// Представь: От [1, 2] до [4, 6] нужно пройти на 3 по первой оси и на 4 по второй; расстояние равно
//   5.
//
// Что изучаем: разности по каждой координате возводим в квадрат, складываем и извлекаем корень.
// Для совпадающих точек ответ 0. Порядок точек не влияет на расстояние.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther;

fn main() {
    let cases: [(&str, &[f64; 2], &[f64; 2], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    for (_description, first_point, second_point, expected) in cases {
        assert!(check_f64_eq_1e_minus_10(calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
                first_point,
                second_point,
            )
            .expect("не удалось вычислить расстояние: координаты должны быть конечными, а квадрат расстояния — помещаться в f64"), expected));
    }
    plot_distance_from_origin_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_distance_from_origin_for_changing_first_coordinate() {
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
                calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
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
