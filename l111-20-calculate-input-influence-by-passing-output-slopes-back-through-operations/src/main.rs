// Урок 20.2. Влияние входа на результат: передача скоростей изменения назад по операциям.
// Связь с принятой терминологией: Передача производной результата назад по вычислительному графу.
// Зачем здесь эта тема: Для обучения нужны производные входов, а не только итоговое значение
//   функции.
// Почему код устроен так: Идём от результата к входам и применяем локальное правило цепочки в
//   каждом узле.
// Представь: Если итоговая ошибка меняется при изменении последнего узла, передаём это влияние
//   назад к x.
//
// Что изучаем: Обратное распространение.
// Зачем это нужно: Правило цепочки передаёт производную результата назад через каждую операцию графа.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input_value: f64 = 2.0;
    let square: f64 = input_value * input_value;
    let _output: f64 = 2.0 * square;
    let derivative_output_by_square: f64 = 2.0;
    let derivative_square_by_input: f64 = 2.0 * input_value;
    let _derivative_output_by_input: f64 = derivative_output_by_square * derivative_square_by_input;

    plot_function_values_and_output_change_per_input_change();
}

// Строим график по результатам урока.
fn plot_function_values_and_output_change_per_input_change() {
    let function_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (horizontal_value, 2.0 * horizontal_value * horizontal_value)
        })
        .collect();
    let derivative_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (horizontal_value, 4.0 * horizontal_value)
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Обратное распространение для 2x²",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "f(x)",

                points: &function_points,
            },
            lesson_visualization::Series {
                name: "df/dx",

                points: &derivative_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
