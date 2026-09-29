// Урок 03.2. Частная производная функции двух переменных.
//
// Что изучаем: Частная производная.
// Зачем это нужно: Для функции двух переменных меняем одну переменную, оставляя другую постоянной. Это
// даёт отдельную производную по каждой оси.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // f(x,y)=x²+3y².
    let (input_value, second_input_value): (f64, f64) = (2.0, -1.0);
    // Умножаем значения и сохраняем результат в `derivative_by_horizontal_coordinate`.
    let derivative_by_horizontal_coordinate: f64 = 2.0 * input_value;
    // Умножаем значения и сохраняем результат в `derivative_by_vertical_coordinate`.
    let derivative_by_vertical_coordinate: f64 = 6.0 * second_input_value;
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        "∂f/∂x={derivative_by_horizontal_coordinate}, ∂f/∂y={derivative_by_vertical_coordinate}"
    );

    // Построение графика вынесено из основного кода урока.
    visualize_partial_derivative_of_two_variable_function();
}

// Строим график по результатам урока.
fn visualize_partial_derivative_of_two_variable_function() {
    // Наглядное представление величин из этого урока.
    let fixed_vertical_coordinate_slice_points: Vec<(f64, f64)> = (-40..=40)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, horizontal_value * horizontal_value + 4.0)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `fixed_horizontal_coordinate_slice_points` в коллекцию.
    let fixed_horizontal_coordinate_slice_points: Vec<(f64, f64)> = (-40..=40)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `vertical_value`.
            let vertical_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (vertical_value, 4.0 + vertical_value * vertical_value)
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
        "Сечения функции x² + 3y²",
        // Указываем подпись горизонтальной оси.
        "координата",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "y=-1",
                // Передаём рассчитанные координаты точек.
                points: &fixed_vertical_coordinate_slice_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "x=2",
                // Передаём рассчитанные координаты точек.
                points: &fixed_horizontal_coordinate_slice_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
