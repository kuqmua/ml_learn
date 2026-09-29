// Сводная практика 11. Метрики бинарной классификации при дисбалансе классов.
//
// Объединяем четыре исхода, precision, recall и F1 из уроков 11.1–11.4.
// При редком положительном классе высокая accuracy может скрывать бесполезную модель.

fn main() {
    // Задаём учебные значения для `labels`.
    let labels: [bool; 10] = [
        // Используем подготовленное значение в следующем шаге примера.
        false, false, false, false, false, false, false, false, false, true,
    ];
    // Задаём учебные значения для `scores`.
    let scores: [f64; 10] = [0.1, 0.2, 0.3, 0.1, 0.2, 0.4, 0.1, 0.3, 0.2, 0.8];
    // Сохраняем результат этого шага в `counts`.
    let counts: part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::BinaryClassificationCounts = part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(&labels, &scores, 0.5).unwrap();
    // Сохраняем результат этого шага в `precision`.
    let precision: Option<f64> =
        part_061_lesson_11_precision_from_binary_classification_counts::precision_from_binary_classification_counts(counts);
    // Сохраняем результат этого шага в `recall`.
    let recall: Option<f64> = part_062_lesson_11_recall_from_binary_classification_counts::recall_from_binary_classification_counts(counts);
    // Сохраняем результат этого шага в `harmonic_mean_of_precision_and_recall`.
    let harmonic_mean_score: Option<f64> = part_063_lesson_11_harmonic_mean_of_binary_classification_precision_and_recall::harmonic_mean_of_precision_and_recall(precision, recall);
    // Сохраняем результат этого шага в `accuracy`.
    let accuracy: Option<f64> =
        part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::calculate_accuracy_from_binary_classification_counts(
            counts,
        );
    // Печатаем рассчитанные значения для проверки примера.
    println!(
        // Передаём подпись или текстовое значение для следующего шага.
        "модель: {counts:?}, precision={precision:?}, recall={recall:?}, F1={harmonic_mean_score:?}, accuracy={accuracy:?}"
    );

    // Задаём учебные значения для `all_negative_scores`.
    let all_negative_scores: [f64; 10] = [0.0; 10];
    // Сохраняем результат этого шага в `useless`.
    let useless: part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::BinaryClassificationCounts =
        // Используем подготовленное значение в следующем шаге примера.
        part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(&labels, &all_negative_scores, 0.5).unwrap();
    // Сохраняем результат этого шага в `useless_accuracy`.
    let useless_accuracy: Option<f64> =
        part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::calculate_accuracy_from_binary_classification_counts(
            useless,
        );
    // Сохраняем результат этого шага в `useless_recall`.
    let useless_recall: Option<f64> =
        part_062_lesson_11_recall_from_binary_classification_counts::recall_from_binary_classification_counts(useless);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(useless_accuracy.unwrap() > 0.8);
    // Проверяем ожидаемое свойство учебного примера.
    assert_eq!(useless_recall, Some(0.0));
    // Печатаем рассчитанные значения для проверки примера.
    println!("всегда отрицательно: accuracy={useless_accuracy:?}, recall={useless_recall:?}");

    // Построение графика вынесено из основного кода урока.
    visualize_binary_classification_metrics_under_class_imbalance(precision, recall, accuracy);
}

// Строим график по результатам урока.
fn visualize_binary_classification_metrics_under_class_imbalance(
    precision: core::option::Option<f64>,
    recall: core::option::Option<f64>,
    accuracy: core::option::Option<f64>,
) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
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
