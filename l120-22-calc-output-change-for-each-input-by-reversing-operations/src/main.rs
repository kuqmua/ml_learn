// Урок 22.1. Вычисление влияния каждого входа на результат обратным проходом по операциям.
// Связь с принятой терминологией: Градиенты скалярного выхода в обратном режиме дифференцирования.
// Зачем здесь эта тема: У скалярной ошибки много входов; обратный режим получает их градиенты одним
//   проходом назад.
// Почему код устроен так: Передаём чувствительность результата по узлам в обратном порядке.
// Представь: Одна итоговая ошибка зависит от многих весов; обратный проход возвращает влияние
//   каждого.
//
// Что изучаем: Обратный режим дифференцирования.
// Зачем это нужно: Один обратный проход вычисляет градиенты скалярного результата по многим входам.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let (input_value, second_input_value): (f64, f64) = (2.0, 3.0);
    let multiplied_coords: f64 = input_value * second_input_value;
    let _: f64 = multiplied_coords + input_value;
    let _: f64 = second_input_value + 1.0;
    let _: f64 = input_value;

    plot_multiply_inputs_then_add_first_with_second_fixed(second_input_value);
}

// Строим график по результатам урока.
fn plot_multiply_inputs_then_add_first_with_second_fixed(vertical_value: f64) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "f(x,y)=xy+x при y=3",
        "x",
        "f(x,3)",
        &[lesson_visualization::Series {
            name: "прямой проход",

            points: &(0..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (
                        horizontal_value,
                        horizontal_value * vertical_value + horizontal_value,
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
