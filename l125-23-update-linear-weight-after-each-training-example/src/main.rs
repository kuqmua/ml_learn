// Урок 23.1. Обновление веса линейной модели после каждого обучающего примера.
// Связь с принятой терминологией: Обновление веса линейной модели после каждого примера методом SGD.
// Зачем здесь эта тема: Полный градиент дорог на большом наборе; SGD обновляет параметры после
//   отдельных строк.
// Почему код устроен так: Показываем порядок примеров и изменение веса после каждого, а не только
//   после эпохи.
// Представь: После первой строки вес изменился, поэтому вторая строка уже видит обновлённую модель.
//
// Что изучаем: Стохастический градиентный спуск.
// Зачем это нужно: SGD обновляет параметр после отдельного примера, поэтому шаги зависят от порядка
// обучающих объектов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let examples: [(f64, f64); 2] = [(1.0, 2.0), (2.0, 4.0)];
    let mut weight: f64 = 0.0;
    let mut weight_history: [(f64, f64); 3] = [(0.0, weight); 3];
    for (step, (feature, target)) in examples.into_iter().enumerate() {
        let rate_of_change: f64 = 2.0 * (weight * feature - target) * feature;
        weight -= 0.1 * rate_of_change;

        weight_history[step + 1] = ((step + 1) as f64, weight);
    }

    plot_weight_after_each_single_example_update(weight_history);
}

// Строим график по результатам урока.
fn plot_weight_after_each_single_example_update(weight_history: [(f64, f64); 3]) {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Шаги SGD",
        "шаг",
        "вес",
        &[lesson_visualization::Series {
            name: "вес",

            points: &weight_history,
        }],
    )
    .expect("не удалось сохранить график");
}
