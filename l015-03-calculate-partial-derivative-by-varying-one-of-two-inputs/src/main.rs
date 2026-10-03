// Урок 03.2. Частная производная: скорость изменения функции при изменении одного из двух входов.
// Связь с принятой терминологией: Частная производная функции двух переменных.
// Зачем здесь эта тема: У модели несколько параметров; нужно знать влияние каждого при неизменных
//   остальных.
// Почему код устроен так: Меняем по очереди одну переменную, чтобы отличить частную производную от
//   общей перемены функции.
// Представь: Если цена зависит от двух чисел, меняем одно и удерживаем другое, чтобы узнать влияние
//   именно первого.
//
// Что изучаем: Частная производная.
// Зачем это нужно: Для функции двух переменных меняем одну переменную, оставляя другую постоянной. Это
// даёт отдельную производную по каждой оси.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let (input_value, second_input_value): (f64, f64) = (2.0, -1.0);
    let _partial_derivative_as_output_change_per_first_input_change_with_second_input_fixed: f64 =
        2.0 * input_value;
    let _partial_derivative_as_output_change_per_second_input_change_with_first_input_fixed: f64 =
        6.0 * second_input_value;

    plot_function_values_while_changing_one_coordinate();
}

// Строим график по результатам урока.
fn plot_function_values_while_changing_one_coordinate() {
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
