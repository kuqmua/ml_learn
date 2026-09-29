// Урок 43.1. Сохранение весов модели в формате с заданными полями.
//
// Что изучаем: Формат весов модели.
// Зачем это нужно: Порядок полей и разделитель должны быть известны при сохранении и чтении параметров.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Сохраняем рассчитанное значение `weight` для следующих операций.
    let weight: f64 = 2.0;
    lesson_trace::trace_step!(weight);
    // Сохраняем рассчитанное значение `bias` для следующих операций.
    let bias: f64 = 1.0;
    lesson_trace::trace_step!(bias);
    // Сохраняем рассчитанное значение `saved_model_text` для следующих операций.
    // Преобразование параметров модели в текст называют serialization.
    let saved_model_text: String = format!("{weight}\n{bias}\n");
    lesson_trace::trace_step!(saved_model_text);
    // Создаём изменяемое значение `lines` для следующих операций.
    let mut lines: std::str::Lines<'_> = saved_model_text.lines();
    lesson_trace::trace_step!(lines);
    // Читаем или разбираем входные данные в значение `loaded_weight`.
    let loaded_weight: f64 = lines.next().unwrap().parse().unwrap();
    lesson_trace::trace_step!(loaded_weight);
    // Читаем или разбираем входные данные в значение `loaded_bias`.
    let loaded_bias: f64 = lines.next().unwrap().parse().unwrap();
    lesson_trace::trace_step!(loaded_bias);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("после загрузки: вес={loaded_weight}, смещение={loaded_bias}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_serialize_model_weights_with_defined_field_format(loaded_weight, loaded_bias);
}

// Строим график по результатам урока.
fn visualize_serialize_model_weights_with_defined_field_format(
    loaded_weight: f64,
    loaded_bias: f64,
) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Загруженные параметры",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[("вес", loaded_weight), ("смещение", loaded_bias)],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
