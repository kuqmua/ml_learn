// Урок 43.1. Сохранение весов модели в заданных полях файла.
// Связь с принятой терминологией: Сохранение весов модели в формате с заданными полями.
// Зачем здесь эта тема: Обученные веса бесполезны после завершения процесса, если их нельзя
//   сохранить.
// Почему код устроен так: Записываем значения в явный формат с именованными полями для последующей
//   загрузки.
// Представь: После перезапуска программы веса должны читаться из файла, а не исчезать вместе с
//   памятью процесса.
//
// Что изучаем: Формат весов модели.
// Зачем это нужно: Порядок полей и разделитель должны быть известны при сохранении и чтении параметров.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `weight` для следующих операций.");
    let weight: f64 = 2.0;
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `bias` для следующих операций.");
    let bias: f64 = 1.0;
    lesson_trace::trace_step!(bias);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `saved_model_text` для следующих операций."
    );
    lesson_trace::trace_note!("Преобразование параметров модели в текст называют serialization.");
    let saved_model_text: String = format!("{weight}\n{bias}\n");
    lesson_trace::trace_step!(saved_model_text);
    lesson_trace::trace_note!("Создаём изменяемое значение `lines` для следующих операций.");
    let mut lines: std::str::Lines<'_> = saved_model_text.lines();
    lesson_trace::trace_step!(lines);
    lesson_trace::trace_note!("Читаем или разбираем входные данные в значение `loaded_weight`.");
    let loaded_weight: f64 = lines.next().unwrap().parse().unwrap();
    lesson_trace::trace_step!(loaded_weight);
    lesson_trace::trace_note!("Читаем или разбираем входные данные в значение `loaded_bias`.");
    let loaded_bias: f64 = lines.next().unwrap().parse().unwrap();
    lesson_trace::trace_step!(loaded_bias);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("после загрузки: вес={loaded_weight}, смещение={loaded_bias}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_weights_loaded_from_saved_model(loaded_weight, loaded_bias);
}

// Строим график по результатам урока.
fn plot_weights_loaded_from_saved_model(loaded_weight: f64, loaded_bias: f64) {
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
        "Загруженные параметры",
        "значение",
        &[("вес", loaded_weight), ("смещение", loaded_bias)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
