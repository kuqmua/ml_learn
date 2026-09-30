// Урок 15.2. Обучение нескольких моделей на разных повторных выборках.
// Связь с принятой терминологией: Обучение моделей на разных выборках с возвращением.
// Зачем здесь эта тема: Разные bootstrap-наборы позволяют получить модели с разными ошибками.
// Почему код устроен так: Обучаем отдельную модель на каждой выборке, не меняя правила построения
//   модели.
// Представь: Два дерева, обученные на разных таких выборках, могут ошибаться на разных объектах.
//
// Что изучаем: Bagging.
// Зачем это нужно: Обучаем несколько моделей на разных bootstrap-выборках, затем объединяем ответы,
// уменьшая зависимость от одной выборки.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `model_predictions` для следующего шага примера.");
    let model_predictions: [bool; 5] = [true, false, true, true, false];
    trace_step!(model_predictions);
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `positive_votes`.");
    let positive_votes: usize = model_predictions.iter().filter(|&&vote| vote).count();
    trace_step!(positive_votes);
    trace_note!("Считаем количество элементов и сохраняем его в `majority_vote_from_models`.");
    trace_note!("Объединение моделей, обученных на разных выборках, называют bagging.");
    let majority_vote_from_models: bool = positive_votes * 2 > model_predictions.len();
    trace_step!(majority_vote_from_models);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("голосов за класс 1: {positive_votes}; ансамбль={majority_vote_from_models}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_votes_of_models_trained_on_resampled_data(model_predictions, positive_votes);
}

// Строим график по результатам урока.
fn plot_votes_of_models_trained_on_resampled_data(
    model_predictions: [bool; 5],
    positive_votes: usize,
) {
    trace_note!("Сравниваем величины, вычисленные в примере.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Голосование bagging",
        "голоса",
        &[
            ("класс 1", positive_votes as f64),
            ("класс 0", (model_predictions.len() - positive_votes) as f64),
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
