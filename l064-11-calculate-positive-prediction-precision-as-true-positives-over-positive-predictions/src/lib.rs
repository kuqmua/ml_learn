//! Урок 064. Точность положительных прогнозов: доля верных среди всех положительных прогнозов.

/// Доля верных среди положительных прогнозов.
/// Точность положительных прогнозов (precision): верные положительные / все положительные прогнозы.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions_where_1_means_no_false_alarms_and_0_means_all_false_alarms(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let predicted_positives: usize = counts.true_positives_as_correctly_detected_positive_cases
        + counts.false_positives_as_false_alarms_on_negative_cases;
    if predicted_positives == 0 {
        None
    } else {
        Some(
            counts.true_positives_as_correctly_detected_positive_cases as f64
                / predicted_positives as f64,
        )
    }
}
