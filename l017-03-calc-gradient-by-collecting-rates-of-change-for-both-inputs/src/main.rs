// Урок 017. Собираем в список скорости изменения ответа по каждому входному числу.
// Каждое число списка отвечает: как изменится результат, если слегка увеличить
// именно этот вход, оставив остальные на месте.
// Если результат — ошибка, положительное число подсказывает уменьшать этот вход,
// отрицательное — увеличивать. Для уменьшения ошибки делаем небольшой шаг.
// Такой список скоростей изменения обычно называют градиентом.

fn main() {
    let (input_value, second_input_value): (f64, f64) = (0.0, 0.0);
    let _gradient: [f64; 2] = [2.0 * (input_value - 2.0), 6.0 * (second_input_value + 1.0)];

    plot_rate_of_change_along_first_coord();
}

// Строим график по результатам урока.
fn plot_rate_of_change_along_first_coord() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Градиент квадратичной функции",
        "x",
        "производная",
        &[lesson_visualization::Series {
            name: "∂f/∂x при y=0",

            points: &(-40..=40)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (horizontal_value, 2.0 * (horizontal_value - 3.0))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
