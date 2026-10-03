// Урок 016. Если вычисление состоит из нескольких шагов, изменение проходит через каждый шаг.
// Здесь сначала считаем 2*x + 1, затем умножаем полученное число само на себя.
// Первый шаг меняется со скоростью 2, второй — со скоростью 2*(2*x + 1).
// Перемножаем эти скорости, чтобы узнать влияние маленького изменения исходного x.
// При x=3 получаем 2*14 = 28.

fn main() {
    let input_value: f64 = 3.0;
    let inner: f64 = 2.0 * input_value + 1.0;
    let outer_derivative: f64 = 2.0 * inner;
    let inner_derivative: f64 = 2.0;
    let _composed_function_slope_as_output_change_per_original_input_change: f64 =
        outer_derivative * inner_derivative;

    plot_fourth_power_and_its_rate_of_change();
}

// Строим график по результатам урока.
fn plot_fourth_power_and_its_rate_of_change() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Правило цепочки: (2x+1)²",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "(2x+1)²",

                points: &(-20..=20)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (
                            horizontal_value,
                            horizontal_value
                                * horizontal_value
                                * horizontal_value
                                * horizontal_value,
                        )
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "производная",

                points: &(-20..=20)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (
                            horizontal_value,
                            4.0 * horizontal_value * horizontal_value * horizontal_value,
                        )
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
