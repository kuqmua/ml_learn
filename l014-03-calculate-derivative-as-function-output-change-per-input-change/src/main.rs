// Урок 03.1. Производная функции: скорость изменения результата при изменении входа.
// Связь с принятой терминологией: Производная функции одной переменной.
// Зачем здесь эта тема: Обучение меняет параметр так, чтобы снизить ошибку; производная показывает
//   местный наклон этой ошибки.
// Почему код устроен так: Начинаем с одной переменной, где знак и величину наклона легко сверить по
//   формуле.
// Представь: Если f(x)=x², при x=3 небольшой рост x увеличивает f; производная 6 показывает местную
//   скорость роста.
//
// Что изучаем: Производная одной переменной.
// Зачем это нужно: Производная показывает мгновенную скорость изменения функции. Для f(x)=x² аналитическая
// производная равна 2x.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `input_value` для следующих операций."
    );
    let input_value: f64 = 3.0;
    lesson_trace::trace_step!(input_value);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `derivative`.");
    let derivative: f64 = 2.0 * input_value;
    lesson_trace::trace_step!(derivative);
    lesson_trace::trace_note!(
        "При x=3 малое увеличение аргумента меняет x² примерно в шесть раз быстрее."
    );
    println!("f(x)=x²; f'(3)={derivative}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_squared_input_and_tangent_line();
}

// Строим график по результатам урока.
fn plot_squared_input_and_tangent_line() {
    lesson_trace::trace_note!("Наглядное представление величин из этого урока.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let function_points: Vec<(f64, f64)> = (0..=60)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (horizontal_value, horizontal_value * horizontal_value)
        })
        .collect();
    lesson_trace::trace_note!("Собираем значения для `tangent_points` в коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let tangent_points: Vec<(f64, f64)> = (0..=60)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (horizontal_value, 9.0 + 6.0 * (horizontal_value - 3.0))
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Функция и касательная в x=3",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "x²",

                points: &function_points,
            },
            lesson_visualization::Series {
                name: "касательная",

                points: &tangent_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
