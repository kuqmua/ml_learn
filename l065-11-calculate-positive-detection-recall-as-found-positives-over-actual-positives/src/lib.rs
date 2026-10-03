//! Урок 065. Полнота обнаружения: доля найденных среди всех действительно положительных примеров.

/// Доля найденных среди действительно положительных объектов.
/// Полнота (recall): найденные положительные / все действительно положительные примеры.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives_where_1_means_all_found_and_0_means_all_missed(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let actual_positives: usize = counts.true_positives_as_correctly_detected_positive_cases
        + counts.false_negatives_as_missed_positive_cases;
    if actual_positives == 0 {
        None
    } else {
        Some(
            counts.true_positives_as_correctly_detected_positive_cases as f64
                / actual_positives as f64,
        )
    }
}
