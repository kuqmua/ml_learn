// Сводная практика 11. Метрики и дисбаланс классов.
//
// Объединяем четыре исхода, precision, recall и F1 из уроков 11.1–11.4.
// При редком положительном классе высокая accuracy может скрывать бесполезную модель.

fn main() {
    // Задаём учебные значения для `labels`.
    let labels = [
        // Используем подготовленное значение в следующем шаге примера.
        false, false, false, false, false, false, false, false, false, true,
    ];
    // Задаём учебные значения для `scores`.
    let scores = [0.1, 0.2, 0.3, 0.1, 0.2, 0.4, 0.1, 0.3, 0.2, 0.8];
    // Сохраняем результат этого шага в `counts`.
    let counts = lesson_058::count_outcomes_at_threshold(&labels, &scores, 0.5).unwrap();
    // Сохраняем результат этого шага в `precision`.
    let precision = lesson_059::precision(counts);
    // Сохраняем результат этого шага в `recall`.
    let recall = lesson_060::recall(counts);
    // Сохраняем результат этого шага в `harmonic_mean_of_precision_and_recall`.
    let harmonic_mean_score = lesson_061::harmonic_mean_of_precision_and_recall(precision, recall);
    // Сохраняем результат этого шага в `accuracy`.
    let accuracy = lesson_058::accuracy(counts);
    // Печатаем рассчитанные значения для проверки примера.
    println!(
        // Передаём подпись или текстовое значение для следующего шага.
        "модель: {counts:?}, precision={precision:?}, recall={recall:?}, F1={harmonic_mean_score:?}, accuracy={accuracy:?}"
    );

    // Задаём учебные значения для `all_negative_scores`.
    let all_negative_scores = [0.0; 10];
    // Сохраняем результат этого шага в `useless`.
    let useless =
        // Используем подготовленное значение в следующем шаге примера.
        lesson_058::count_outcomes_at_threshold(&labels, &all_negative_scores, 0.5).unwrap();
    // Сохраняем результат этого шага в `useless_accuracy`.
    let useless_accuracy = lesson_058::accuracy(useless);
    // Сохраняем результат этого шага в `useless_recall`.
    let useless_recall = lesson_060::recall(useless);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(useless_accuracy.unwrap() > 0.8);
    // Проверяем ожидаемое свойство учебного примера.
    assert_eq!(useless_recall, Some(0.0));
    // Печатаем рассчитанные значения для проверки примера.
    println!("всегда отрицательно: accuracy={useless_accuracy:?}, recall={useless_recall:?}");

    // Построение графика вынесено из основного кода урока.
    visualize(precision, recall, accuracy);
}

// Строим график по результатам урока.
fn visualize(
    precision: core::option::Option<f64>,
    recall: core::option::Option<f64>,
    accuracy: core::option::Option<f64>,
) {
    // Сравнение величин из этого урока.
    let chart = lesson_visualization::bars(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Метрики при дисбалансе классов",
        // Указываем подпись вертикальной оси.
        "доля",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("precision", precision.unwrap_or(0.0)),
            // Добавляем пару значений для сравнения или построения графика.
            ("recall", recall.unwrap_or(0.0)),
            // Добавляем пару значений для сравнения или построения графика.
            ("accuracy", accuracy.unwrap_or(0.0)),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
