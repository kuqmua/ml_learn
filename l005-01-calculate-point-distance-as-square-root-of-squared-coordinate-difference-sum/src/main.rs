// Урок 01.4. Расстояние между точками: квадратный корень из суммы квадратов разностей координат.
// Связь с принятой терминологией: Евклидово расстояние между точками.
// Зачем здесь эта тема: Квадрат расстояния из предыдущей части переводим в обычное расстояние.
// Почему код устроен так: Вызываем предыдущую часть и извлекаем корень стандартным sqrt.
//   Проверку размерностей выполняет функция квадрата расстояния.
// Представь: От [1, 2] до [4, 6] нужно пройти на 3 по первой оси и на 4 по второй; расстояние равно
//   5.
//
// Что изучаем: разности по каждой координате возводим в квадрат, складываем и извлекаем корень.
// Для совпадающих точек ответ 0. Порядок точек не влияет на расстояние.

use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences;

fn main() {
    let cases: [(&str, &[f64], &[f64], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    for (_description, first_point, second_point, expected) in cases {
        let distance: f64 =
            calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                first_point,
                second_point,
            )
            .expect("точки в этом примере имеют одинаковую размерность");
        assert!((distance - expected).abs() < 1e-10);
    }
    let first_point: [f64; 2] = [0.0, 0.0];
    let too_short: [f64; 1] = [3.0];
    let _error: &str =
        calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            &first_point,
            &too_short,
        )
        .expect_err("точки разной размерности нужно отклонить");

    plot_distance_from_origin_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_distance_from_origin_for_changing_first_coordinate() {
    let distance_points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (
                horizontal_value,
                calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                    &[0.0, 0.0],
                    &[horizontal_value, 4.0],
                )
                .unwrap(),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Расстояние до начала координат",
        "x точки [x, 4]",
        "расстояние",
        &[lesson_visualization::Series {
            name: "расстояние",

            points: &distance_points,
        }],
    )
    .expect("не удалось сохранить график");
}
