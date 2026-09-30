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

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Задаём учебные значения для `input`.");
    let input: [f64; 2] = [1.0, 2.0];
    trace_step!(input);
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2], [f64; 2], bool); 2] = [
        ("одинаковые веса", [0.2, -0.3], [0.2, -0.3], true),
        ("разные веса", [0.2, -0.3], [-0.1, 0.4], false),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, first_neuron, second_neuron, should_match) in cases {
        trace_step!(description);
        trace_step!(first_neuron);
        trace_step!(second_neuron);
        trace_step!(should_match);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(first_neuron.len(), input.len());
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(second_neuron.len(), input.len());
        trace_note!("Сохраняем результат этого шага в `first_output`.");
        let first_output: f64 = first_neuron[0] * input[0] + first_neuron[1] * input[1];
        trace_step!(first_output);
        trace_note!("Сохраняем результат этого шага в `second_output`.");
        let second_output: f64 = second_neuron[0] * input[0] + second_neuron[1] * input[1];
        trace_step!(second_output);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(first_output == second_output, should_match);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: ответы {first_output} и {second_output}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_different_starting_weights();
}

// Строим график по результатам урока.
fn plot_different_starting_weights() {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Разные начальные веса",
        "вес",
        &[("нейрон 1", 0.5), ("нейрон 2", -0.5)],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
