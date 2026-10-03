// Урок 01.3. Длина вектора: квадратный корень из суммы квадратов координат.
// Связь с принятой терминологией: Вычисление евклидовой длины одного вектора.
// Зачем здесь эта тема: Для расстояний нужна геометрическая длина; сумма квадратов из скалярного
//   произведения даёт её квадрат.
// Почему код устроен так: Берём корень из произведения вектора на самого себя; пары 3–4–5 позволяют
//   проверить результат вручную.
// Представь: Для [3, 4] квадраты дают 9+16=25, а корень возвращает длину 5.
//
// Что изучаем: длина вектора — корень из суммы квадратов координат.
// Смена знаков длину не меняет; длина нулевого вектора равна нулю.

use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer;

fn main() {
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("обычный вектор", [3.0, 4.0], 5.0),
        ("сменили знаки", [-3.0, -4.0], 5.0),
        ("вдвое длиннее", [6.0, 8.0], 10.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (_description, vector, expected) in cases {
        assert!(
            (calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer(&vector)
                - expected)
                .abs()
                < 1e-10
        );
    }

    plot_vector_length_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_vector_length_for_changing_first_coordinate() {
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
                        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer(&[
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
