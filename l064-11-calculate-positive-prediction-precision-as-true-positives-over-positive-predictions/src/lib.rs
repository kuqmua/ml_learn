//! Урок 064. Точность положительных прогнозов: доля верных среди всех положительных прогнозов.

/// Доля верных среди положительных прогнозов.
/// Точность положительных прогнозов (precision): верные положительные / все положительные прогнозы.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let predicted_positives: usize = counts.true_positives + counts.false_positives;
    if predicted_positives == 0 {
        None
    } else {
        Some(counts.true_positives as f64 / predicted_positives as f64)
    }
}
