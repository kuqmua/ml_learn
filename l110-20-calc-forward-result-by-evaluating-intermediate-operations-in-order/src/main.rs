// Урок 20.1. Прямой расчёт результата: вычисление промежуточных операций по порядку.
// Зачем здесь эта тема: Сложную функцию удобно представить цепью простых операций; это основа
//   автоматического дифференцирования.
// Почему код устроен так: Сначала считаем значения узлов в порядке зависимостей, сохраняя
//   промежуточные результаты.
// Представь: В цепочке x→x²→2x² каждый узел считает результат только после готовности входа.
//
// Что изучаем: Прямой проход графа.
// Зачем это нужно: Каждый узел вычисляет значение из уже готовых входов; здесь f(x)=tanh(2x²) заменяем
// простой составной функцией 2x² для изоляции прямого прохода.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input_value: f64 = 2.0;
    let square: f64 = input_value * input_value;
    let _: f64 = square + square;

    plot_intermediate_values_of_squared_input_computation();
}

// Строим график по результатам урока.
fn plot_intermediate_values_of_squared_input_computation() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Прямой проход вычислительного графа",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "x²",

                points: &(-30..=30)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, horizontal_value * horizontal_value)
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "2x²",

                points: &(-30..=30)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, 2.0 * horizontal_value * horizontal_value)
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
