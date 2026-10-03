// Урок 014. Узнаём, насколько меняется ответ при очень маленьком изменении входа.
// Для формулы x*x скорость изменения в точке x равна 2*x.
// При x=3 это 6: если увеличить вход на 0.001, ответ вырастет примерно на 0.006.
// Это местная оценка для маленького изменения, а не обещание для любого шага.
// Такую скорость изменения называют производной. Она пригодится для уменьшения ошибки.

fn main() {
    let input_value: f64 = 3.0;
    let _derivative_as_local_output_change_per_input_change: f64 = 2.0 * input_value;

    plot_squared_input_and_tangent_line();
}

// Строим график по результатам урока.
fn plot_squared_input_and_tangent_line() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Функция и касательная в x=3",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "x²",

                points: &(0..=60)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, horizontal_value * horizontal_value)
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "касательная",

                points: &(0..=60)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, 9.0 + 6.0 * (horizontal_value - 3.0))
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
