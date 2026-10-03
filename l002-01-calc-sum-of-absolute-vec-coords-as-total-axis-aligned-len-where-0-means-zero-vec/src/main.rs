// Урок 002. Складываем величины чисел, не учитывая их знаки.
// Для [-3, 4] получаем 3 + 4 = 7. Минус не уменьшает результат.
// Это длина пути, если двигаться только вдоль осей: сначала на 3, затем на 4.
// Длина прямого пути считается иначе — в следующем уроке.

use l002_01_calc_sum_of_absolute_vec_coords_as_total_axis_aligned_len_where_0_means_zero_vec::calc_sum_of_absolute_vec_coords_as_total_axis_aligned_len_where_0_means_zero_vec;

fn main() {
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("положительные координаты", [3.0, 4.0], 7.0),
        ("смешанные знаки", [3.0, -4.0], 7.0),
        ("сменили оба знака", [-3.0, 4.0], 7.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (_description, vec, expected) in cases {
        assert_eq!(
            calc_sum_of_absolute_vec_coords_as_total_axis_aligned_len_where_0_means_zero_vec(&vec),
            expected
        );
    }

    plot_sum_of_absolute_coords_for_changing_first_coord();
}

// Строим график по результатам урока.
fn plot_sum_of_absolute_coords_for_changing_first_coord() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сумма модулей координат одного вектора",
        "первая координата",
        "сумма модулей",
        &[lesson_visualization::Series {
            name: "вектор [x, 4]",

            points: &(-50..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (
                        horizontal_value,
                        calc_sum_of_absolute_vec_coords_as_total_axis_aligned_len_where_0_means_zero_vec(&[horizontal_value, 4.0]),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
