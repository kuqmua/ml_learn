// Урок 03.3. Производная вложенных функций: умножение скоростей изменения по правилу цепочки.
// Связь с принятой терминологией: Правило цепочки для производной композиции функций.
// Зачем здесь эта тема: Модель обычно состоит из последовательных преобразований; изменение входа
//   проходит через каждое из них.
// Почему код устроен так: Расписываем внутреннюю и внешнюю производные отдельно, затем перемножаем
//   их по правилу цепочки.
// Представь: Если x сначала превращается в x², а затем результат умножается на 3, изменение x
//   проходит через оба шага.
//
// Что изучаем: Правило цепочки.
// Зачем это нужно: Производную композиции получаем умножением производной внешней функции на производную
// внутренней.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("f(x)=(2x+1)²: внутренняя функция u=2x+1, внешняя u².");
    let input_value: f64 = 3.0;
    trace_step!(input_value);
    trace_note!("Умножаем значения и сохраняем результат в `inner`.");
    let inner: f64 = 2.0 * input_value + 1.0;
    trace_step!(inner);
    trace_note!("Умножаем значения и сохраняем результат в `outer_derivative`.");
    let outer_derivative: f64 = 2.0 * inner;
    trace_step!(outer_derivative);
    trace_note!("Сохраняем рассчитанное значение `inner_derivative` для следующих операций.");
    let inner_derivative: f64 = 2.0;
    trace_step!(inner_derivative);
    trace_note!("Умножаем значения и сохраняем результат в `derivative`.");
    let derivative: f64 = outer_derivative * inner_derivative;
    trace_step!(derivative);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("f'(3)={derivative}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_fourth_power_and_its_rate_of_change();
}

// Строим график по результатам урока.
fn plot_fourth_power_and_its_rate_of_change() {
    trace_note!("Наглядное представление величин из этого урока.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let composed_function_points: Vec<(f64, f64)> = (-20..=20)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                horizontal_value,
                horizontal_value * horizontal_value * horizontal_value * horizontal_value,
            )
        })
        .collect();
    trace_note!("Собираем значения для `derivative_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let derivative_points: Vec<(f64, f64)> = (-20..=20)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                horizontal_value,
                4.0 * horizontal_value * horizontal_value * horizontal_value,
            )
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
        "Правило цепочки: (2x+1)²",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "(2x+1)²",

                points: &composed_function_points,
            },
            lesson_visualization::Series {
                name: "производная",

                points: &derivative_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
