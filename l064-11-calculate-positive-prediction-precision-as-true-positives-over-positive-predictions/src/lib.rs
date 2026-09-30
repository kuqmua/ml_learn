//! Урок 064. Точность положительных прогнозов: доля верных среди всех положительных прогнозов.

/// Доля верных среди положительных прогнозов.
/// Точность положительных прогнозов (precision): верные положительные / все положительные прогнозы.
pub fn calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(
    counts: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts,
) -> Option<f64> {
    lesson_trace::trace_note!("Сохраняем результат этого шага в `predicted_positives`.");
    let predicted_positives: usize = counts.true_positives + counts.false_positives;
    lesson_trace::trace_step!(predicted_positives);
    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if predicted_positives == 0 {
        lesson_trace::trace_note!("Отмечаем отсутствие подходящего значения.");
        None
    } else {
        lesson_trace::trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
        lesson_trace::trace_note!("Возвращаем присутствующее значение.");
        Some(counts.true_positives as f64 / predicted_positives as f64)
    }
}
