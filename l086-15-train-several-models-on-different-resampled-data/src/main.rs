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

fn main() {
    let model_predictions: [bool; 5] = [true, false, true, true, false];
    let pos_votes: usize = model_predictions.iter().filter(|&&vote| vote).count();
    let _: bool = pos_votes * 2 > model_predictions.len();

    plot_votes_of_models_trained_on_resampled_data(model_predictions, pos_votes);
}

// Строим график по результатам урока.
fn plot_votes_of_models_trained_on_resampled_data(model_predictions: [bool; 5], pos_votes: usize) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Голосование bagging",
        "голоса",
        &[
            ("класс 1", pos_votes as f64),
            ("класс 0", (model_predictions.len() - pos_votes) as f64),
        ],
    )
    .expect("не удалось сохранить график");
}
