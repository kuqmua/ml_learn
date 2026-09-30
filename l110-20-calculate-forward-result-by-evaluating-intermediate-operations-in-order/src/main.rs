// Урок 20.1. Прямой расчёт результата: вычисление промежуточных операций по порядку.
// Связь с принятой терминологией: Вычисление значений узлов при прямом проходе вычислительного графа.
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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Сохраняем рассчитанное значение `input_value` для следующих операций.");
    let input_value: f64 = 2.0;
    trace_step!(input_value);
    trace_note!("Умножаем значения и сохраняем результат в `square`.");
    let square: f64 = input_value * input_value;
    trace_step!(square);
    trace_note!("Комбинируем исходные величины и сохраняем результат в `doubled_square`.");
    let doubled_square: f64 = square + square;
    trace_step!(doubled_square);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("x={input_value}, x²={square}, 2x²={doubled_square}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_intermediate_values_of_squared_input_computation();
}

// Строим график по результатам урока.
fn plot_intermediate_values_of_squared_input_computation() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let squared_function_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, horizontal_value * horizontal_value)
        })
        .collect();
    trace_note!("Собираем значения для `doubled_squared_function_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let doubled_squared_function_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, 2.0 * horizontal_value * horizontal_value)
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Прямой проход вычислительного графа",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "x²",

                points: &squared_function_points,
            },
            lesson_visualization::Series {
                name: "2x²",

                points: &doubled_squared_function_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
