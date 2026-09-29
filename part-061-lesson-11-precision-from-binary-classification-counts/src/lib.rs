//! Вычисления и примеры урока part-061-lesson-11-precision-from-binary-classification-counts.

/// Доля верных среди положительных прогнозов.
pub fn precision_from_binary_classification_counts(
    counts: part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::BinaryClassificationCounts,
) -> Option<f64> {
    // Сохраняем результат этого шага в `predicted_positives`.
    let predicted_positives: usize = counts.true_positives + counts.false_positives;
    lesson_trace::trace_step!(predicted_positives);
    // Выбираем дальнейший шаг по выполнению условия.
    if predicted_positives == 0 {
        // Отмечаем отсутствие подходящего значения.
        None
    // Обрабатываем случай, когда предыдущее условие не выполнено.
    } else {
        // Возвращаем присутствующее значение.
        Some(counts.true_positives as f64 / predicted_positives as f64)
    }
}
