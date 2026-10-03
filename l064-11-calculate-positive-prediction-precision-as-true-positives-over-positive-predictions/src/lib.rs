//! Урок 064. Точность положительных прогнозов: доля верных среди всех положительных прогнозов.

/// Доля верных среди положительных прогнозов.
/// Точность положительных прогнозов (precision): верные положительные / все положительные прогнозы.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;

pub fn calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions_where_1_means_no_false_alarms_and_0_means_all_false_alarms(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    let predicted_poss: usize = counts.true_poss_as_correctly_detected_pos_cases
        + counts.false_poss_as_false_alarms_on_neg_cases;
    if predicted_poss == 0 {
        None
    } else {
        Some(counts.true_poss_as_correctly_detected_pos_cases as f64 / predicted_poss as f64)
    }
}
