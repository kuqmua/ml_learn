// Урок 32.1. Построение контекста каждой позиции по позициям той же последовательности.
// Связь с принятой терминологией: Внимание между токенами одной последовательности.
// Зачем здесь эта тема: Когда Q, K и V приходят из одной последовательности, токены могут
//   обмениваться контекстом.
// Почему код устроен так: Сравниваем выход токена до и после взвешивания других позиций.
// Представь: Слово может изменить своё представление после того, как получит сведения от других
//   слов этой же фразы.
//
// Что изучаем: Self-attention.
// Зачем это нужно: Токены одной последовательности сравниваются друг с другом, чтобы каждый выход учёл
// подходящий контекст.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `text_units` для следующего шага примера.");
    lesson_trace::trace_note!(
        "Единицу текста, которую модель обрабатывает как одно целое, называют token."
    );
    let text_units: [f64; 2] = [1.0, 3.0];
    lesson_trace::trace_step!(text_units);
    lesson_trace::trace_note!("Создаём набор значений `weights` для следующего шага примера.");
    let weights: [[f64; 2]; 2] = [[0.8, 0.2], [0.4, 0.6]];
    lesson_trace::trace_step!(weights);
    lesson_trace::trace_note!("Создаём набор значений `context` для следующего шага примера.");
    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
    let context: [f64; 2] = [
        weights[0][0] * text_units[0] + weights[0][1] * text_units[1],
        weights[1][0] * text_units[0] + weights[1][1] * text_units[1],
    ];
    lesson_trace::trace_step!(context);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("контекст для двух позиций = {context:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_weighted_context_coordinates(context);
}

// Строим график по результатам урока.
fn plot_weighted_context_coordinates(context: [f64; 2]) {
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
        "Self-attention: контекст",
        "компонента",
        &[("позиция 0", context[0]), ("позиция 1", context[1])],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
