// Урок 003. Считаем длину стрелки от начала координат до заданной точки.
// Каждое число умножаем само на себя, складываем результаты и берём квадратный корень.
// Для [3, 4]: 3×3 + 4×4 = 25, корень из 25 равен 5, потому что 5×5 = 25.
// Смена знака координаты меняет направление, но не длину.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l003_01_calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer::calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer;

fn main() {
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("обычный вектор", [3.0, 4.0], 5.0),
        ("сменили знаки", [-3.0, -4.0], 5.0),
        ("вдвое длиннее", [6.0, 8.0], 10.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (_description, vec, expected) in cases {
        assert!(
            check_f64_eq_1e_minus_10(calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer(&vec), expected)
        );
    }

    plot_vec_len_for_changing_first_coord();
}

// Строим график по результатам урока.
fn plot_vec_len_for_changing_first_coord() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Длина вектора (норма L2)",
        "первая координата",
        "длина вектора",
        &[lesson_visualization::Series {
            name: "вектор [x, 4]",

            points: &(-50..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (
                        horizontal_value,
                        calc_vec_len_as_square_root_of_sum_of_squared_coords_where_0_means_zero_vec_and_larger_means_longer(&[
                            horizontal_value,
                            4.0,
                        ]),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
