// Урок 22.1. Обратный режим дифференцирования.
//
// Что изучаем: Обратный режим дифференцирования.
// Зачем это нужно: Один обратный проход вычисляет градиенты скалярного результата по многим входам.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // f(x,y)=x*y+x.
    let (input_value, second_input_value) = (2.0, 3.0);
    // Умножаем значения и сохраняем результат в `multiplied_coordinates`.
    let multiplied_coordinates = input_value * second_input_value;
    // Комбинируем исходные величины и сохраняем результат в `output`.
    let output = multiplied_coordinates + input_value;
    // Комбинируем исходные величины и сохраняем результат в `derivative_by_horizontal_coordinate`.
    let derivative_by_horizontal_coordinate = second_input_value + 1.0;
    // Сохраняем рассчитанное значение `derivative_by_vertical_coordinate` для следующих операций.
    let derivative_by_vertical_coordinate = input_value;
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        "f={output}, df/dx={derivative_by_horizontal_coordinate}, df/dy={derivative_by_vertical_coordinate}"
    );

    // Построение графика вынесено из основного кода урока.
    visualize(second_input_value);
}

// Строим график по результатам урока.
fn visualize(vertical_value: f64) {
    // Собираем значения для `chart_points` в коллекцию.
    let chart_points: Vec<(f64, f64)> = (0..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (
                horizontal_value,
                horizontal_value * vertical_value + horizontal_value,
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart = lesson_visualization::line_chart(
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
