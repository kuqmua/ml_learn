//! Урок 063. Доля верных прогнозов: число правильных ответов, делённое на общее число.

/// Общая доля верных прогнозов.
/// Доля правильных прогнозов (accuracy): (верные положительные + верные отрицательные) / все прогнозы.
pub fn calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
    counts: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts,
) -> Option<f64> {
    lesson_trace::trace_note!("Сохраняем результат этого шага в `total`.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    let total: usize = counts.true_positives
        + counts.false_positives
        + counts.true_negatives
        + counts.false_negatives;
    lesson_trace::trace_step!(total);
    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if total == 0 {
        lesson_trace::trace_note!("Отмечаем отсутствие подходящего значения.");
        None
    } else {
        lesson_trace::trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
        lesson_trace::trace_note!("Возвращаем присутствующее значение.");
        Some((counts.true_positives + counts.true_negatives) as f64 / total as f64)
    }
}
