//! Урок 066. Оценка F1: удвоенное произведение точности и полноты, делённое на их сумму.

/// Гармоническое среднее precision и recall.
/// F1 — гармоническое среднее precision и recall: 2·precision·recall / (precision + recall).

pub fn calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
    correct_pos_prediction_share: Option<f64>,
    actual_pos_detection_share: Option<f64>,
) -> Option<f64> {
    match (correct_pos_prediction_share, actual_pos_detection_share) {
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
