// Урок 09.5. Качество на отложенных данных.
//
// Что изучаем: Качество на отложенных данных.
// Зачем это нужно: Train служит для выбора параметров; качество модели оцениваем на новых примерах, не
// участвовавших в обучении.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Создаём набор значений `training` для следующего шага примера.
    let training = [(1.0, 3.0), (2.0, 5.0)];
    // Создаём набор значений `test` для следующего шага примера.
    let test = [(3.0, 7.0), (4.0, 9.0)];
    assert!(
        training.len() >= 2,
        "для прямой нужны хотя бы две обучающие точки"
    );
    assert_ne!(
        training[0].0, training[1].0,
        "обучающие точки должны иметь разные значения x"
    );
    assert!(
        !test.is_empty(),
        "для MSE нужен хотя бы один тестовый пример"
    );
    // Нормируем или усредняем величину делением и сохраняем её в `weight`.
    let weight = (training[1].1 - training[0].1) / (training[1].0 - training[0].0);
    // Умножаем значения и сохраняем результат в `bias`.
    let bias = training[0].1 - weight * training[0].0;
    let targets: Vec<_> = test.iter().map(|&(_, target)| target).collect();
    let predictions: Vec<_> = test
        .iter()
        .map(|&(feature, _)| weight * feature + bias)
        .collect();
    let mse = lesson_047::mean_squared_error(&targets, &predictions).unwrap();
    println!("test MSE = {mse}");
    // Показываем значения, рассчитанные по данным примера.
    let chart_points_0: Vec<(f64, f64)> = training.iter().map(|&(x, y)| (x, y)).collect();
    let chart_points_1: Vec<(f64, f64)> = test.iter().map(|&(x, y)| (x, y)).collect();
    let chart_points_2: Vec<(f64, f64)> = (0..=50)
        .map(|i| {
            let x = i as f64 / 10.0;
            (x, weight * x + bias)
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Отложенные данные и прямая",
        "признак",
        "цель и прогноз",
        &[
            lesson_visualization::Series {
                name: "обучение",
                points: &chart_points_0,
            },
            lesson_visualization::Series {
                name: "тест",
                points: &chart_points_1,
            },
            lesson_visualization::Series {
                name: "модель",
                points: &chart_points_2,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
