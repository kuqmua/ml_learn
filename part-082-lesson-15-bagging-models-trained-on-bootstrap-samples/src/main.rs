// Урок 15.2. Обучение моделей на разных выборках с возвращением.
// Зачем здесь эта тема: Разные bootstrap-наборы позволяют получить модели с разными ошибками.
// Почему код устроен так: Обучаем отдельную модель на каждой выборке, не меняя правила построения
//   модели.
// Представь: Два дерева, обученные на разных таких выборках, могут ошибаться на разных объектах.
//
// Что изучаем: Bagging.
// Зачем это нужно: Обучаем несколько моделей на разных bootstrap-выборках, затем объединяем ответы,
// уменьшая зависимость от одной выборки.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `model_predictions` для следующего шага примера.
    let model_predictions: [bool; 5] = [true, false, true, true, false];
    lesson_trace::trace_step!(model_predictions);
    // Преобразуем входные данные и сохраняем полученную коллекцию в `positive_votes`.
    let positive_votes: usize = model_predictions.iter().filter(|&&vote| vote).count();
    lesson_trace::trace_step!(positive_votes);
    // Считаем количество элементов и сохраняем его в `majority_vote_from_models`.
    // Объединение моделей, обученных на разных выборках, называют bagging.
    let majority_vote_from_models: bool = positive_votes * 2 > model_predictions.len();
    lesson_trace::trace_step!(majority_vote_from_models);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("голосов за класс 1: {positive_votes}; ансамбль={majority_vote_from_models}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_bagging_models_trained_on_bootstrap_samples(model_predictions, positive_votes);
}

// Строим график по результатам урока.
fn visualize_bagging_models_trained_on_bootstrap_samples(
    model_predictions: [bool; 5],
    positive_votes: usize,
) {
    // Сравниваем величины, вычисленные в примере.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Голосование bagging",
        // Указываем подпись вертикальной оси.
        "голоса",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("класс 1", positive_votes as f64),
            // Добавляем пару значений для сравнения или построения графика.
            ("класс 0", (model_predictions.len() - positive_votes) as f64),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
