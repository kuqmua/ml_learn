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
    lesson_trace::enable();
    lesson_trace::trace_note!("f(x,y)=x*y+x.");
    let (input_value, second_input_value): (f64, f64) = (2.0, 3.0);
    lesson_trace::trace_step!(input_value);
    lesson_trace::trace_step!(second_input_value);
    lesson_trace::trace_note!(
        "Умножаем значения и сохраняем результат в `multiplied_coordinates`."
    );
    let multiplied_coordinates: f64 = input_value * second_input_value;
    lesson_trace::trace_step!(multiplied_coordinates);
    lesson_trace::trace_note!("Комбинируем исходные величины и сохраняем результат в `output`.");
    let output: f64 = multiplied_coordinates + input_value;
    lesson_trace::trace_step!(output);
    lesson_trace::trace_note!(
        "Комбинируем исходные величины и сохраняем результат в `derivative_by_horizontal_coordinate`."
    );
    let derivative_by_horizontal_coordinate: f64 = second_input_value + 1.0;
    lesson_trace::trace_step!(derivative_by_horizontal_coordinate);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `derivative_by_vertical_coordinate` для следующих операций."
    );
    let derivative_by_vertical_coordinate: f64 = input_value;
    lesson_trace::trace_step!(derivative_by_vertical_coordinate);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!(
        "f={output}, df/dx={derivative_by_horizontal_coordinate}, df/dy={derivative_by_vertical_coordinate}"
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_input_product_plus_first_input_with_second_fixed(second_input_value);
}

// Строим график по результатам урока.
fn plot_input_product_plus_first_input_with_second_fixed(vertical_value: f64) {
    lesson_trace::trace_note!("Собираем значения для `chart_points` в коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let chart_points: Vec<(f64, f64)> = (0..=50)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                horizontal_value,
                horizontal_value * vertical_value + horizontal_value,
            )
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "f(x,y)=xy+x при y=3",
        "x",
        "f(x,3)",
        &[lesson_visualization::Series {
            name: "прямой проход",

            points: &chart_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
