// Урок 01.3. Длина вектора: квадратный корень из суммы квадратов координат.
// Связь с принятой терминологией: Вычисление евклидовой длины одного вектора.
// Зачем здесь эта тема: Для расстояний нужна геометрическая длина; сумма квадратов из скалярного
//   произведения даёт её квадрат.
// Почему код устроен так: Берём корень из произведения вектора на самого себя; пары 3–4–5 позволяют
//   проверить результат вручную.
// Представь: Для [3, 4] квадраты дают 9+16=25, а корень возвращает длину 5.
//
// Что изучаем: длина вектора — корень из суммы квадратов координат.
// Смена знаков длину не меняет; длина нулевого вектора равна нулю.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("обычный вектор", [3.0, 4.0], 5.0),
        ("сменили знаки", [-3.0, -4.0], 5.0),
        ("вдвое длиннее", [6.0, 8.0], 10.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, vector, expected) in cases {
        trace_step!(description);
        trace_step!(vector);
        trace_step!(expected);
        trace_note!(
            "Длина вектора — корень из суммы квадратов координат: для [3, 4] это sqrt(9 + 16) = 5."
        );
        let length: f64 =
            l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(
                &vector,
            );
        trace_step!(length);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((length - expected).abs() < 1e-10);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {vector:?} → длина = {length}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_vector_length_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_vector_length_for_changing_first_coordinate() {
    trace_note!("Наглядное представление величин из этого урока.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let vector_length_points: Vec<(f64, f64)> = (-50..=50)

        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                horizontal_value,
                l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(&[
                    horizontal_value,
                    4.0,
                ]),
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
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Длина вектора (норма L2)",
        "первая координата",
        "длина вектора",
        &[lesson_visualization::Series {
            name: "вектор [x, 4]",

            points: &vector_length_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
