//! Урок 066. Оценка F1: удвоенное произведение точности и полноты, делённое на их сумму.

/// Гармоническое среднее precision и recall.
/// F1 — гармоническое среднее precision и recall: 2·precision·recall / (precision + recall).

pub fn calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(
    correct_pos_prediction_share_where_1_means_no_false_alarms: Option<f64>,
    actual_pos_detection_share_where_1_means_none_missed: Option<f64>,
) -> Option<f64> {
    match (
        correct_pos_prediction_share_where_1_means_no_false_alarms,
        actual_pos_detection_share_where_1_means_none_missed,
    ) {
        (Some(correct_pos_prediction_share), Some(actual_pos_detection_share))
            if correct_pos_prediction_share + actual_pos_detection_share > 0.0 =>
        {
            Some(
                2.0 * correct_pos_prediction_share * actual_pos_detection_share
                    / (correct_pos_prediction_share + actual_pos_detection_share),
            )
        }

        _ => None,
    }
}
