// Урок 018. Проверяем скорость изменения без заранее выведенной формулы.
// Считаем ответ при x + step и при x - step. Разницу ответов делим на 2*step.
// Так узнаём среднее изменение ответа на единицу входа в маленьком промежутке около x.
// Сравниваем оценку с известной скоростью 2*x для формулы x*x.
// Слишком маленький step может ухудшить ответ из-за округления дробных чисел.

fn main() {
    let input_value: f64 = 3.0;
    let step: f64 = 0.0001;
    let value_after_adding_step: f64 = (input_value + step) * (input_value + step);
    let value_after_subtracting_step: f64 = (input_value - step) * (input_value - step);
    let _estimated_derivative_as_local_output_change_per_input_change: f64 =
        (value_after_adding_step - value_after_subtracting_step) / (2.0 * step);
    let analytical_derivative: f64 = 2.0 * input_value;

    plot_slope_estimation_error_for_shrinking_step(input_value, analytical_derivative);
}

// Строим график по результатам урока.
fn plot_slope_estimation_error_for_shrinking_step(
    horizontal_value: f64,
    analytical_derivative: f64,
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ошибка центральной разности",
        "k для h=10⁻ᵏ",
        "абсолютная ошибка",
        &[lesson_visualization::Series {
            name: "x² в x=3",

            points: &(1..=12)
                .map(|step_exponent| {
                    let step_size: f64 = 10f64.powi(-step_exponent);
                    let numeric: f64 = ((horizontal_value + step_size)
                        * (horizontal_value + step_size)
                        - (horizontal_value - step_size) * (horizontal_value - step_size))
                        / (2.0 * step_size);
                    (
                        step_exponent as f64,
                        (numeric - analytical_derivative).abs(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
