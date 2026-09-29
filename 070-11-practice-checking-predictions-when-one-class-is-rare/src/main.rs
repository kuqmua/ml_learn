// Урок 11.7. Практика: оценка прогнозов, когда один класс встречается редко.
// Связь с принятой терминологией: Метрики бинарной классификации при дисбалансе классов.
// Зачем здесь эта тема: Для несбалансированных классов accuracy, precision, recall и AUC отвечают
//   на разные вопросы.
// Почему код устроен так: Считаем их рядом на одних прогнозах, чтобы не выбирать метрику по
//   удобному числу.
// Представь: Модель «всегда отрицательно» может иметь высокую accuracy, но нулевой recall редкого
//   класса.
//
// Объединяем четыре исхода, precision, recall и F1 из уроков 11.1–11.4.
// При редком положительном классе высокая accuracy может скрывать бесполезную модель.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `labels`.
    let labels: [bool; 10] = [
        // Используем подготовленное значение в следующем шаге примера.
        false, false, false, false, false, false, false, false, false, true,
    ];
    lesson_trace::trace_step!(labels);
    // Задаём учебные значения для `scores`.
    let scores: [f64; 10] = [0.1, 0.2, 0.3, 0.1, 0.2, 0.4, 0.1, 0.3, 0.2, 0.8];
    lesson_trace::trace_step!(scores);
    // Сохраняем результат этого шага в `counts`.
    let counts: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts = l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(&labels, &scores, 0.5).unwrap();
    lesson_trace::trace_step!(counts);
    // Сохраняем результат этого шага в `precision`.
    let precision: Option<f64> =
        l064_11_calculate_positive_prediction_precision_as_true_positives_over_positive_predictions::calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(counts);
    lesson_trace::trace_step!(precision);
    // Сохраняем результат этого шага в `recall`.
    let recall: Option<f64> = l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts);
    lesson_trace::trace_step!(recall);
    // Сохраняем результат этого шага в `calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum`.
    let harmonic_mean_score: Option<f64> = l066_11_calculate_f1_score_as_twice_precision_times_recall_over_their_sum::calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(precision, recall);
    lesson_trace::trace_step!(harmonic_mean_score);
    // Сохраняем результат этого шага в `accuracy`.
    let accuracy: Option<f64> =
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            counts,
        );
    lesson_trace::trace_step!(accuracy);
    // Печатаем рассчитанные значения для проверки примера.
    println!(
        // Передаём подпись или текстовое значение для следующего шага.
        "модель: {counts:?}, precision={precision:?}, recall={recall:?}, F1={harmonic_mean_score:?}, accuracy={accuracy:?}"
    );

    // Задаём учебные значения для `all_negative_scores`.
    let all_negative_scores: [f64; 10] = [0.0; 10];
    lesson_trace::trace_step!(all_negative_scores);
    // Сохраняем результат этого шага в `useless`.
    let useless: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts =
        // Используем подготовленное значение в следующем шаге примера.
        l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(&labels, &all_negative_scores, 0.5).unwrap();
    lesson_trace::trace_step!(useless);
    // Сохраняем результат этого шага в `useless_accuracy`.
    let useless_accuracy: Option<f64> =
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            useless,
        );
    lesson_trace::trace_step!(useless_accuracy);
    // Сохраняем результат этого шага в `useless_recall`.
    let useless_recall: Option<f64> =
        l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(useless);
    lesson_trace::trace_step!(useless_recall);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(useless_accuracy.unwrap() > 0.8);
    // Проверяем ожидаемое свойство учебного примера.
    assert_eq!(useless_recall, Some(0.0));
    // Печатаем рассчитанные значения для проверки примера.
    println!("всегда отрицательно: accuracy={useless_accuracy:?}, recall={useless_recall:?}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_prediction_quality_shares_for_imbalanced_classes(precision, recall, accuracy);
}

// Строим график по результатам урока.
fn plot_prediction_quality_shares_for_imbalanced_classes(
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
