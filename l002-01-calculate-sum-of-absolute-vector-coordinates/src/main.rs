// Урок 01.2. Длина пути вдоль осей (норма L1): сложение модулей координат вектора.
// Связь с принятой терминологией: Сумма модулей координат одного вектора (норма L1).
// Зачем здесь эта тема: После произведения векторов нужна величина одного вектора; L1 измеряет
//   общий модуль координат без взаимного сокращения знаков.
// Почему код устроен так: Складываем модули явно, чтобы отрицательные координаты не уменьшали
//   норму.
// Представь: Если координаты равны −3 и 4, общий размер по L1 равен 3+4=7: минус не уменьшает
//   ответ.
//
// Что изучаем: складываем модули всех координат. Отрицательное число даёт положительный вклад,
// поэтому смена знаков не меняет ответ. У нулевого вектора результат равен нулю.

use l002_01_calculate_sum_of_absolute_vector_coordinates::calculate_sum_of_absolute_vector_coordinates_as_total_axis_aligned_length_where_0_means_zero_vector;

fn main() {
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("положительные координаты", [3.0, 4.0], 7.0),
        ("смешанные знаки", [3.0, -4.0], 7.0),
        ("сменили оба знака", [-3.0, 4.0], 7.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (_description, vector, expected) in cases {
        assert_eq!(
            calculate_sum_of_absolute_vector_coordinates_as_total_axis_aligned_length_where_0_means_zero_vector(&vector),
            expected
        );
    }

    plot_sum_of_absolute_coordinates_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_sum_of_absolute_coordinates_for_changing_first_coordinate() {
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
                        calculate_sum_of_absolute_vector_coordinates_as_total_axis_aligned_length_where_0_means_zero_vector(&[horizontal_value, 4.0]),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
