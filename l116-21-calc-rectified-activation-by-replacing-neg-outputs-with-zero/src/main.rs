// Урок 116. Пропускаем положительные числа и заменяем отрицательные на ноль.
// Например, из [-2, 0, 3] получаем [0, 0, 3].
// Такой шаг позволяет цепочке слоёв описывать более сложные зависимости,
// чем одно умножение входов на веса с последующим сложением.

fn main() {
    for raw_model_score in [-2.0, 0.0, 2.0] {
        let _relu_output: f64 = if raw_model_score > 0.0 {
            raw_model_score
        } else {
            0.0
        };
    }

    plot_rectified_activation_as_input_with_neg_values_replaced_by_zero();
}

// Строим график по результатам урока.
fn plot_rectified_activation_as_input_with_neg_values_replaced_by_zero() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "ReLU",
        "вход",
        "выход",
        &[lesson_visualization::Series {
            name: "max(0,x)",

            points: &(-50..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (horizontal_value, horizontal_value.max(0.0))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
