//! Урок 063. Доля верных прогнозов: число правильных ответов, делённое на общее число.

/// Общая доля верных прогнозов.
/// Доля правильных прогнозов (accuracy): (верные положительные + верные отрицательные) / все прогнозы.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let all_prediction_count: usize = counts.true_positives
        + counts.false_positives
        + counts.true_negatives
        + counts.false_negatives;
    if all_prediction_count == 0 {
        None
    } else {
        Some((counts.true_positives + counts.true_negatives) as f64 / all_prediction_count as f64)
    }
}
