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
    let weight: f64 = 2.0;
    let constant_input_weight: f64 = 1.0;
    let saved_model_text: String = format!("{weight}\n{constant_input_weight}\n");
    let mut lines: std::str::Lines<'_> = saved_model_text.lines();
    let loaded_weight: f64 = lines.next().unwrap().parse().unwrap();
    let loaded_constant_input_weight: f64 = lines.next().unwrap().parse().unwrap();

    plot_weights_loaded_from_saved_model(loaded_weight, loaded_constant_input_weight);
}

// Строим график по результатам урока.
fn plot_weights_loaded_from_saved_model(loaded_weight: f64, loaded_constant_input_weight: f64) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Загруженные параметры",
        "значение",
        &[
            ("вес", loaded_weight),
            ("смещение", loaded_constant_input_weight),
        ],
    )
    .expect("не удалось сохранить график");
}
