// Урок 15.1. Повторные обучающие выборки: набор примеров с возвращением и возможными повторами.
// Связь с принятой терминологией: Выборка с возвращением для обучения ансамбля.
// Зачем здесь эта тема: Одно дерево может сильно зависеть от обучающих строк; выборки с
//   возвращением дают разные наборы для ансамбля.
// Почему код устроен так: Берём случайные индексы с возвращением, поэтому строка может повториться
//   или не попасть в выборку.
// Представь: Из строк [A, B, C] выборка с возвращением может стать [A, A, C]; строка B в неё не
//   попадёт.
//
// Что изучаем: Bootstrap-выборка.
// Зачем это нужно: Случайная выборка с возвращением может содержать один объект несколько раз и пропускать
// другой.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `data` для следующего шага примера.");
    let data: [&str; 4] = ["A", "B", "C", "D"];
    lesson_trace::trace_step!(data);
    lesson_trace::trace_note!(
        "Создаём набор значений `sampled_indices` для следующего шага примера."
    );
    let sampled_indices: [usize; 4] = [0, 2, 2, 3];
    lesson_trace::trace_step!(sampled_indices);
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `sample`."
    );
    let sample: Vec<&str> = sampled_indices.iter().map(|&index| data[index]).collect();
    lesson_trace::trace_step!(sample);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("исходные={data:?}, bootstrap={sample:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_repeated_appearances_in_sample_drawn_with_replacement(sample);
}

// Строим график по результатам урока.
fn plot_repeated_appearances_in_sample_drawn_with_replacement(sample: std::vec::Vec<&str>) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Повторы в bootstrap-выборке",
        "число появлений",
        &[
            (
                "A",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "A")
                    .count() as f64,
            ),
            (
                "B",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "B")
                    .count() as f64,
            ),
            (
                "C",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "C")
                    .count() as f64,
            ),
            (
                "D",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "D")
                    .count() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
