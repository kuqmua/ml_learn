//! Урок 065. Полнота обнаружения: доля найденных среди всех действительно положительных примеров.

/// Доля найденных среди действительно положительных объектов.
/// Полнота (recall): найденные положительные / все действительно положительные примеры.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let actual_positives: usize = counts.true_positives + counts.false_negatives;
    if actual_positives == 0 {
        None
    } else {
        Some(counts.true_positives as f64 / actual_positives as f64)
    }
}
