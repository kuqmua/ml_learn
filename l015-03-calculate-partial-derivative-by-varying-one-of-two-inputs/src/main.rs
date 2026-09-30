// Урок 03.2. Частная производная: скорость изменения функции при изменении одного из двух входов.
// Связь с принятой терминологией: Частная производная функции двух переменных.
// Зачем здесь эта тема: У модели несколько параметров; нужно знать влияние каждого при неизменных
//   остальных.
// Почему код устроен так: Меняем по очереди одну переменную, чтобы отличить частную производную от
//   общей перемены функции.
// Представь: Если цена зависит от двух чисел, меняем одно и удерживаем другое, чтобы узнать влияние
//   именно первого.
//
// Что изучаем: Частная производная.
// Зачем это нужно: Для функции двух переменных меняем одну переменную, оставляя другую постоянной. Это
// даёт отдельную производную по каждой оси.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("f(x,y)=x²+3y².");
    let (input_value, second_input_value): (f64, f64) = (2.0, -1.0);
    trace_step!(input_value);
    trace_step!(second_input_value);
    trace_note!("Умножаем значения и сохраняем результат в `derivative_by_horizontal_coordinate`.");
    let derivative_by_horizontal_coordinate: f64 = 2.0 * input_value;
    trace_step!(derivative_by_horizontal_coordinate);
    trace_note!("Умножаем значения и сохраняем результат в `derivative_by_vertical_coordinate`.");
    let derivative_by_vertical_coordinate: f64 = 6.0 * second_input_value;
    trace_step!(derivative_by_vertical_coordinate);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!(
        "∂f/∂x={derivative_by_horizontal_coordinate}, ∂f/∂y={derivative_by_vertical_coordinate}"
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_function_values_while_changing_one_coordinate();
}

// Строим график по результатам урока.
fn plot_function_values_while_changing_one_coordinate() {
    trace_note!("Наглядное представление величин из этого урока.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let fixed_vertical_coordinate_slice_points: Vec<(f64, f64)> = (-40..=40)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, horizontal_value * horizontal_value + 4.0)
        })
        .collect();
    trace_note!("Собираем значения для `fixed_horizontal_coordinate_slice_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let fixed_horizontal_coordinate_slice_points: Vec<(f64, f64)> = (-40..=40)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `vertical_value`.");
            let vertical_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (vertical_value, 4.0 + vertical_value * vertical_value)
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
        "Сечения функции x² + 3y²",
        "координата",
        "значение",
        &[
            lesson_visualization::Series {
                name: "y=-1",

                points: &fixed_vertical_coordinate_slice_points,
            },
            lesson_visualization::Series {
                name: "x=2",

                points: &fixed_horizontal_coordinate_slice_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
