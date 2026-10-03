//! Урок 065. Полнота обнаружения: доля найденных среди всех действительно положительных примеров.

/// Доля найденных среди действительно положительных объектов.
/// Полнота (recall): найденные положительные / все действительно положительные примеры.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let actual_poss: usize =
        counts.true_poss_as_correctly_detected_pos_cases + counts.false_negs_as_missed_pos_cases;
    if actual_poss == 0 {
        None
    } else {
        Some(counts.true_poss_as_correctly_detected_pos_cases as f64 / actual_poss as f64)
    }
}
