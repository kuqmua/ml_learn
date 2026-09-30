//! Урок 065. Полнота обнаружения: доля найденных среди всех действительно положительных примеров.

/// Доля найденных среди действительно положительных объектов.
/// Полнота (recall): найденные положительные / все действительно положительные примеры.
use lesson_trace::{trace_note, trace_step};

pub fn calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(
    counts: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts,
) -> Option<f64> {
    trace_note!("Сохраняем результат этого шага в `actual_positives`.");
    let actual_positives: usize = counts.true_positives + counts.false_negatives;
    trace_step!(actual_positives);
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if actual_positives == 0 {
        trace_note!("Отмечаем отсутствие подходящего значения.");
        None
    } else {
        trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
        trace_note!("Возвращаем присутствующее значение.");
        Some(counts.true_positives as f64 / actual_positives as f64)
    }
}
