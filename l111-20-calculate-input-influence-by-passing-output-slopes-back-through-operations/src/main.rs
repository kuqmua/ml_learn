// Урок 20.2. Влияние входа на результат: передача скоростей изменения назад по операциям.
// Связь с принятой терминологией: Передача производной результата назад по вычислительному графу.
// Зачем здесь эта тема: Для обучения нужны производные входов, а не только итоговое значение
//   функции.
// Почему код устроен так: Идём от результата к входам и применяем локальное правило цепочки в
//   каждом узле.
// Представь: Если итоговая ошибка меняется при изменении последнего узла, передаём это влияние
//   назад к x.
//
// Что изучаем: Обратное распространение.
// Зачем это нужно: Правило цепочки передаёт производную результата назад через каждую операцию графа.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Сохраняем рассчитанное значение `input_value` для следующих операций.");
    let input_value: f64 = 2.0;
    trace_step!(input_value);
    trace_note!("Умножаем значения и сохраняем результат в `square`.");
    let square: f64 = input_value * input_value;
    trace_step!(square);
    trace_note!("Умножаем значения и сохраняем результат в `output`.");
    let output: f64 = 2.0 * square;
    trace_step!(output);
    trace_note!(
        "Сохраняем рассчитанное значение `derivative_output_by_square` для следующих операций."
    );
    let derivative_output_by_square: f64 = 2.0;
    trace_step!(derivative_output_by_square);
    trace_note!("Умножаем значения и сохраняем результат в `derivative_square_by_input`.");
    let derivative_square_by_input: f64 = 2.0 * input_value;
    trace_step!(derivative_square_by_input);
    trace_note!("Умножаем значения и сохраняем результат в `derivative_output_by_input`.");
    let derivative_output_by_input: f64 = derivative_output_by_square * derivative_square_by_input;
    trace_step!(derivative_output_by_input);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("f(x)={output}, df/dx={derivative_output_by_input}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_function_values_and_output_change_per_input_change();
}

// Строим график по результатам урока.
fn plot_function_values_and_output_change_per_input_change() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let function_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, 2.0 * horizontal_value * horizontal_value)
        })
        .collect();
    trace_note!("Собираем значения для `derivative_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let derivative_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, 4.0 * horizontal_value)
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
        "Обратное распространение для 2x²",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "f(x)",

                points: &function_points,
            },
            lesson_visualization::Series {
                name: "df/dx",

                points: &derivative_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
