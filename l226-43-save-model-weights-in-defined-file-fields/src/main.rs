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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Сохраняем рассчитанное значение `weight` для следующих операций.");
    let weight: f64 = 2.0;
    trace_step!(weight);
    trace_note!("Сохраняем рассчитанное значение `bias` для следующих операций.");
    let bias: f64 = 1.0;
    trace_step!(bias);
    trace_note!("Сохраняем рассчитанное значение `saved_model_text` для следующих операций.");
    trace_note!("Преобразование параметров модели в текст называют serialization.");
    let saved_model_text: String = format!("{weight}\n{bias}\n");
    trace_step!(saved_model_text);
    trace_note!("Создаём изменяемое значение `lines` для следующих операций.");
    let mut lines: std::str::Lines<'_> = saved_model_text.lines();
    trace_step!(lines);
    trace_note!("Читаем или разбираем входные данные в значение `loaded_weight`.");
    let loaded_weight: f64 = lines.next().unwrap().parse().unwrap();
    trace_step!(loaded_weight);
    trace_note!("Читаем или разбираем входные данные в значение `loaded_bias`.");
    let loaded_bias: f64 = lines.next().unwrap().parse().unwrap();
    trace_step!(loaded_bias);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("после загрузки: вес={loaded_weight}, смещение={loaded_bias}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_weights_loaded_from_saved_model(loaded_weight, loaded_bias);
}

// Строим график по результатам урока.
fn plot_weights_loaded_from_saved_model(loaded_weight: f64, loaded_bias: f64) {
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
        "Загруженные параметры",
        "значение",
        &[("вес", loaded_weight), ("смещение", loaded_bias)],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
