// Урок 20.2. Передача производной результата назад по вычислительному графу.
//
// Что изучаем: Обратное распространение.
// Зачем это нужно: Правило цепочки передаёт производную результата назад через каждую операцию графа.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Сохраняем рассчитанное значение `input_value` для следующих операций.
    let input_value: f64 = 2.0;
    lesson_trace::trace_step!(input_value);
    // Умножаем значения и сохраняем результат в `square`.
    let square: f64 = input_value * input_value;
    lesson_trace::trace_step!(square);
    // Умножаем значения и сохраняем результат в `output`.
    let output: f64 = 2.0 * square;
    lesson_trace::trace_step!(output);
    // Сохраняем рассчитанное значение `derivative_output_by_square` для следующих операций.
    let derivative_output_by_square: f64 = 2.0;
    lesson_trace::trace_step!(derivative_output_by_square);
    // Умножаем значения и сохраняем результат в `derivative_square_by_input`.
    let derivative_square_by_input: f64 = 2.0 * input_value;
    lesson_trace::trace_step!(derivative_square_by_input);
    // Умножаем значения и сохраняем результат в `derivative_output_by_input`.
    let derivative_output_by_input: f64 = derivative_output_by_square * derivative_square_by_input;
    lesson_trace::trace_step!(derivative_output_by_input);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("f(x)={output}, df/dx={derivative_output_by_input}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_backpropagate_output_derivative_through_computation_graph();
}

// Строим график по результатам урока.
fn visualize_backpropagate_output_derivative_through_computation_graph() {
    // График величин и зависимостей, изученных в этом уроке.
    let function_points: Vec<(f64, f64)> = (-30..=30)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, 2.0 * horizontal_value * horizontal_value)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `derivative_points` в коллекцию.
    let derivative_points: Vec<(f64, f64)> = (-30..=30)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, 4.0 * horizontal_value)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Обратное распространение для 2x²",
        // Указываем подпись горизонтальной оси.
        "x",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "f(x)",
                // Передаём рассчитанные координаты точек.
                points: &function_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "df/dx",
                // Передаём рассчитанные координаты точек.
                points: &derivative_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
