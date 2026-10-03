//! Урок 067. F1 из счётчиков: вычисление точности и полноты и их гармонического среднего.

/// Получаем обе метрики из счётчиков предыдущих уроков.
/// F1 из счётчиков: находим precision и recall, затем делим их удвоенное произведение на сумму.
use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l064_11_calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions::calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions;
use l065_11_calc_pos_detection_recall_as_true_poss_divided_by_actual_poss::calc_pos_detection_recall_as_true_poss_divided_by_actual_poss;
use l066_11_calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum::calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum;

pub fn calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
        calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions(counts),
        calc_pos_detection_recall_as_true_poss_divided_by_actual_poss(counts),
    )
}
