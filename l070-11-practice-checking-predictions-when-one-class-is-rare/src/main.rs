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

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `labels`.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    let labels: [bool; 10] = [
        false, false, false, false, false, false, false, false, false, true,
    ];
    trace_step!(labels);
    trace_note!("Задаём учебные значения для `scores`.");
    let scores: [f64; 10] = [0.1, 0.2, 0.3, 0.1, 0.2, 0.4, 0.1, 0.3, 0.2, 0.8];
    trace_step!(scores);
    trace_note!("Сохраняем результат этого шага в `counts`.");
    let counts: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts = l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(&labels, &scores, 0.5).unwrap();
    trace_step!(counts);
    trace_note!("Сохраняем результат этого шага в `precision`.");
    let precision: Option<f64> =
        l064_11_calculate_positive_prediction_precision_as_true_positives_over_positive_predictions::calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(counts);
    trace_step!(precision);
    trace_note!("Сохраняем результат этого шага в `recall`.");
    let recall: Option<f64> = l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts);
    trace_step!(recall);
    trace_note!(
        "Сохраняем результат этого шага в `calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum`."
    );
    let harmonic_mean_score: Option<f64> = l066_11_calculate_f1_score_as_twice_precision_times_recall_over_their_sum::calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(precision, recall);
    trace_step!(harmonic_mean_score);
    trace_note!("Сохраняем результат этого шага в `accuracy`.");
    let accuracy: Option<f64> =
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            counts,
        );
    trace_step!(accuracy);
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    println!(
        "модель: {counts:?}, precision={precision:?}, recall={recall:?}, F1={harmonic_mean_score:?}, accuracy={accuracy:?}"
    );

    trace_note!("Задаём учебные значения для `all_negative_scores`.");
    let all_negative_scores: [f64; 10] = [0.0; 10];
    trace_step!(all_negative_scores);
    trace_note!("Сохраняем результат этого шага в `useless`.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    let useless: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts =

        l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(&labels, &all_negative_scores, 0.5).unwrap();
    trace_step!(useless);
    trace_note!("Сохраняем результат этого шага в `useless_accuracy`.");
    let useless_accuracy: Option<f64> =
        l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            useless,
        );
    trace_step!(useless_accuracy);
    trace_note!("Сохраняем результат этого шага в `useless_recall`.");
    let useless_recall: Option<f64> =
        l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(useless);
    trace_step!(useless_recall);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(useless_accuracy.unwrap() > 0.8);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert_eq!(useless_recall, Some(0.0));
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("всегда отрицательно: accuracy={useless_accuracy:?}, recall={useless_recall:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_prediction_quality_shares_for_imbalanced_classes(precision, recall, accuracy);
}

// Строим график по результатам урока.
fn plot_prediction_quality_shares_for_imbalanced_classes(
    precision: core::option::Option<f64>,
    recall: core::option::Option<f64>,
    accuracy: core::option::Option<f64>,
) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Метрики при дисбалансе классов",
        "доля",
        &[
            ("precision", precision.unwrap_or(0.0)),
            ("recall", recall.unwrap_or(0.0)),
            ("accuracy", accuracy.unwrap_or(0.0)),
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
