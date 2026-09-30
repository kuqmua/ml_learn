// Урок 21.4. Различие нейронов при обучении: назначение разных начальных весов.
// Связь с принятой терминологией: Инициализация разных весов нейронов перед обучением.
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
    lesson_trace::trace_note!("Задаём учебные значения для `input`.");
    let input: [f64; 2] = [1.0, 2.0];
    lesson_trace::trace_step!(input);
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2], [f64; 2], bool); 2] = [
        ("одинаковые веса", [0.2, -0.3], [0.2, -0.3], true),
        ("разные веса", [0.2, -0.3], [-0.1, 0.4], false),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, first_neuron, second_neuron, should_match) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first_neuron);
        lesson_trace::trace_step!(second_neuron);
        lesson_trace::trace_step!(should_match);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(first_neuron.len(), input.len());
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(second_neuron.len(), input.len());
        lesson_trace::trace_note!("Сохраняем результат этого шага в `first_output`.");
        let first_output: f64 = first_neuron[0] * input[0] + first_neuron[1] * input[1];
        lesson_trace::trace_step!(first_output);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `second_output`.");
        let second_output: f64 = second_neuron[0] * input[0] + second_neuron[1] * input[1];
        lesson_trace::trace_step!(second_output);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(first_output == second_output, should_match);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: ответы {first_output} и {second_output}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_different_starting_weights();
}

// Строим график по результатам урока.
fn plot_different_starting_weights() {
    lesson_trace::trace_note!("Сравнение величин из этого урока.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Разные начальные веса",
        "вес",
        &[("нейрон 1", 0.5), ("нейрон 2", -0.5)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
