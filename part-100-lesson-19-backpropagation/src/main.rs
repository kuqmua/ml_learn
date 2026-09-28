// Урок 19.2. Обратное распространение.
//
// Что изучаем: Обратное распространение.
// Зачем это нужно: Правило цепочки передаёт производную результата назад через каждую операцию графа.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Сохраняем рассчитанное значение `x` для следующих операций.
    let x = 2.0;
    // Умножаем значения и сохраняем результат в `square`.
    let square = x * x;
    // Умножаем значения и сохраняем результат в `output`.
    let output = 2.0 * square;
    // Сохраняем рассчитанное значение `derivative_output_by_square` для следующих операций.
    let derivative_output_by_square = 2.0;
    // Умножаем значения и сохраняем результат в `derivative_square_by_x`.
    let derivative_square_by_x = 2.0 * x;
    // Умножаем значения и сохраняем результат в `derivative_output_by_x`.
    let derivative_output_by_x = derivative_output_by_square * derivative_square_by_x;
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("f(x)={output}, df/dx={derivative_output_by_x}");
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (-30..=30)
        .map(|i| {
            let x = i as f64 / 10.0;
            (x, 2.0 * x * x)
        })
        .collect();
    let chart_points_1: Vec<(f64, f64)> = (-30..=30)
        .map(|i| {
            let x = i as f64 / 10.0;
            (x, 4.0 * x)
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Обратное распространение для 2x²",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "f(x)",
                points: &chart_points_0,
            },
            lesson_visualization::Series {
                name: "df/dx",
                points: &chart_points_1,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
