// Урок 03.3. Производная вложенных функций: умножение скоростей изменения по правилу цепочки.
// Связь с принятой терминологией: Правило цепочки для производной композиции функций.
// Зачем здесь эта тема: Модель обычно состоит из последовательных преобразований; изменение входа
//   проходит через каждое из них.
// Почему код устроен так: Расписываем внутреннюю и внешнюю производные отдельно, затем перемножаем
//   их по правилу цепочки.
// Представь: Если x сначала превращается в x², а затем результат умножается на 3, изменение x
//   проходит через оба шага.
//
// Что изучаем: Правило цепочки.
// Зачем это нужно: Производную композиции получаем умножением производной внешней функции на производную
// внутренней.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input_value: f64 = 3.0;
    let inner: f64 = 2.0 * input_value + 1.0;
    let outer_derivative: f64 = 2.0 * inner;
    let inner_derivative: f64 = 2.0;
    let _derivative: f64 = outer_derivative * inner_derivative;

    plot_fourth_power_and_its_rate_of_change();
}

// Строим график по результатам урока.
fn plot_fourth_power_and_its_rate_of_change() {
    let composed_function_points: Vec<(f64, f64)> = (-20..=20)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (
                horizontal_value,
                horizontal_value * horizontal_value * horizontal_value * horizontal_value,
            )
        })
        .collect();
    let derivative_points: Vec<(f64, f64)> = (-20..=20)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (
                horizontal_value,
                4.0 * horizontal_value * horizontal_value * horizontal_value,
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Правило цепочки: (2x+1)²",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "(2x+1)²",

                points: &composed_function_points,
            },
            lesson_visualization::Series {
                name: "производная",

                points: &derivative_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
