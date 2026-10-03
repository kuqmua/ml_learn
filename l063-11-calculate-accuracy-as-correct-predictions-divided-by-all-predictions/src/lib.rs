//! Урок 063. Доля верных прогнозов: число правильных ответов, делённое на общее число.

/// Общая доля верных прогнозов.
/// Доля правильных прогнозов (accuracy): (верные положительные + верные отрицательные) / все прогнозы.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let all_prediction_count: usize = counts.true_positives_as_correctly_detected_positive_cases
        + counts.false_positives_as_false_alarms_on_negative_cases
        + counts.true_negatives_as_correctly_rejected_negative_cases
        + counts.false_negatives_as_missed_positive_cases;
    if all_prediction_count == 0 {
        None
    } else {
        Some(
            (counts.true_positives_as_correctly_detected_positive_cases
                + counts.true_negatives_as_correctly_rejected_negative_cases) as f64
                / all_prediction_count as f64,
        )
    }
}
