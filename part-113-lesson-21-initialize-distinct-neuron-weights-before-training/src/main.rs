// Урок 21.4. Инициализация разных весов нейронов перед обучением.
// Зачем здесь эта тема: Одинаковые стартовые веса нейронов сохраняют симметрию и мешают учить
//   разные признаки.
// Почему код устроен так: Задаём разные начальные веса и сравниваем их дальнейшие выходы.
// Представь: Если два нейрона начали с одинаковых весов и получают одинаковые градиенты, они
//   останутся одинаковыми.
//
// Нейроны с одинаковыми весами дают одинаковый ответ на один вход.
// Разные веса позволяют нейронам начать обучение с разных ответов.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `input`.
    let input: [f64; 2] = [1.0, 2.0];
    lesson_trace::trace_step!(input);
    // Задаём учебные значения для `cases`.
    let cases: [(&str, [f64; 2], [f64; 2], bool); 2] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("одинаковые веса", [0.2, -0.3], [0.2, -0.3], true),
        // Добавляем пару значений для сравнения или построения графика.
        ("разные веса", [0.2, -0.3], [-0.1, 0.4], false),
    ];
    lesson_trace::trace_step!(cases);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, first_neuron, second_neuron, should_match) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first_neuron);
        lesson_trace::trace_step!(second_neuron);
        lesson_trace::trace_step!(should_match);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(first_neuron.len(), input.len());
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(second_neuron.len(), input.len());
        // Сохраняем результат этого шага в `first_output`.
        let first_output: f64 = first_neuron[0] * input[0] + first_neuron[1] * input[1];
        lesson_trace::trace_step!(first_output);
        // Сохраняем результат этого шага в `second_output`.
        let second_output: f64 = second_neuron[0] * input[0] + second_neuron[1] * input[1];
        lesson_trace::trace_step!(second_output);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(first_output == second_output, should_match);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: ответы {first_output} и {second_output}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_initialize_distinct_neuron_weights_before_training();
}

// Строим график по результатам урока.
fn visualize_initialize_distinct_neuron_weights_before_training() {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Разные начальные веса",
        // Указываем подпись вертикальной оси.
        "вес",
        // Передаём ряды или значения для отрисовки графика.
        &[("нейрон 1", 0.5), ("нейрон 2", -0.5)],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
