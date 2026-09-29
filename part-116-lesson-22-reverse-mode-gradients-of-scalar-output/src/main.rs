// Урок 22.1. Градиенты скалярного выхода в обратном режиме дифференцирования.
// Почему этот урок сейчас: У скалярной ошибки много входов; обратный режим получает их градиенты одним проходом назад.
// Почему пример устроен так: Передаём чувствительность результата по узлам в обратном порядке.
//
// Что изучаем: Обратный режим дифференцирования.
// Зачем это нужно: Один обратный проход вычисляет градиенты скалярного результата по многим входам.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // f(x,y)=x*y+x.
    let (input_value, second_input_value): (f64, f64) = (2.0, 3.0);
    lesson_trace::trace_step!(input_value);
    lesson_trace::trace_step!(second_input_value);
    // Умножаем значения и сохраняем результат в `multiplied_coordinates`.
    let multiplied_coordinates: f64 = input_value * second_input_value;
    lesson_trace::trace_step!(multiplied_coordinates);
    // Комбинируем исходные величины и сохраняем результат в `output`.
    let output: f64 = multiplied_coordinates + input_value;
    lesson_trace::trace_step!(output);
    // Комбинируем исходные величины и сохраняем результат в `derivative_by_horizontal_coordinate`.
    let derivative_by_horizontal_coordinate: f64 = second_input_value + 1.0;
    lesson_trace::trace_step!(derivative_by_horizontal_coordinate);
    // Сохраняем рассчитанное значение `derivative_by_vertical_coordinate` для следующих операций.
    let derivative_by_vertical_coordinate: f64 = input_value;
    lesson_trace::trace_step!(derivative_by_vertical_coordinate);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        "f={output}, df/dx={derivative_by_horizontal_coordinate}, df/dy={derivative_by_vertical_coordinate}"
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_reverse_mode_gradients_of_scalar_output(second_input_value);
}

// Строим график по результатам урока.
fn visualize_reverse_mode_gradients_of_scalar_output(vertical_value: f64) {
    // Собираем значения для `chart_points` в коллекцию.
    let chart_points: Vec<(f64, f64)> = (0..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (
                horizontal_value,
                horizontal_value * vertical_value + horizontal_value,
            )
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
        "f(x,y)=xy+x при y=3",
        // Указываем подпись горизонтальной оси.
        "x",
        // Указываем подпись вертикальной оси.
        "f(x,3)",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "прямой проход",
            // Передаём рассчитанные координаты точек.
            points: &chart_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
