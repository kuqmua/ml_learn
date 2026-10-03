// Урок 015. Когда ответ зависит от двух чисел, рассматриваем влияние каждого отдельно.
// Для x*x + 3*y*y при изменении x удерживаем y на месте: скорость изменения равна 2*x.
// При изменении y удерживаем x на месте: скорость изменения равна 6*y.
// Так можно узнать, какой вход и насколько влияет на ответ рядом с выбранной точкой.

fn main() {
    let (input_value, second_input_value): (f64, f64) = (2.0, -1.0);
    let _partial_derivative_as_output_change_per_first_input_change_with_second_input_fixed: f64 =
        2.0 * input_value;
    let _partial_derivative_as_output_change_per_second_input_change_with_first_input_fixed: f64 =
        6.0 * second_input_value;

    plot_function_values_while_changing_one_coord();
}

// Строим график по результатам урока.
fn plot_function_values_while_changing_one_coord() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сечения функции x² + 3y²",
        "координата",
        "значение",
        &[
            lesson_visualization::Series {
                name: "y=-1",

                points: &(-40..=40)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, horizontal_value * horizontal_value + 4.0)
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "x=2",

                points: &(-40..=40)
                    .map(|plot_step_index| {
                        let vertical_value: f64 = plot_step_index as f64 / 10.0;
                        (vertical_value, 4.0 + vertical_value * vertical_value)
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
