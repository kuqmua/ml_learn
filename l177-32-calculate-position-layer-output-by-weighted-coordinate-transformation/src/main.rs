// Урок 32.4. Выход слоя для каждой позиции: взвешенное преобразование координат.
// Связь с принятой терминологией: Преобразование каждого токена полносвязным слоем.
// Зачем здесь эта тема: Внимание смешивает сведения между позициями, но каждой позиции нужно и
//   собственное нелинейное преобразование.
// Почему код устроен так: Применяем одинаковый полносвязный блок к каждому токену независимо.
// Представь: После обмена сведениями между словами каждый вектор отдельно проходит одинаковую
//   нелинейную функцию.
//
// Что изучаем: Feed-forward слой.
// Зачем это нужно: После внимания каждый токен отдельно проходит через линейное преобразование и
// нелинейную активацию.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `text_unit` для следующего шага примера.");
    lesson_trace::trace_note!(
        "Единицу текста, которую модель обрабатывает как одно целое, называют token."
    );
    let text_unit: [f64; 2] = [1.0, 2.0];
    lesson_trace::trace_step!(text_unit);
    lesson_trace::trace_note!("Создаём набор значений `linear` для следующего шага примера.");
    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
    let linear: [f64; 2] = [
        0.5 * text_unit[0] - 0.2 * text_unit[1],
        0.3 * text_unit[0] + 0.4 * text_unit[1],
    ];
    lesson_trace::trace_step!(linear);
    lesson_trace::trace_note!("Создаём набор значений `activated` для следующего шага примера.");
    lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
    lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
    let activated: [f64; 2] = [
        if linear[0] > 0.0 { linear[0] } else { 0.0 },
        if linear[1] > 0.0 { linear[1] } else { 0.0 },
    ];
    lesson_trace::trace_step!(activated);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("после feed-forward = {activated:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_activations_after_transforming_each_position(activated);
}

// Строим график по результатам урока.
fn plot_activations_after_transforming_each_position(activated: [f64; 2]) {
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
        "Feed-forward",
        "активация",
        &[("нейрон 0", activated[0]), ("нейрон 1", activated[1])],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
