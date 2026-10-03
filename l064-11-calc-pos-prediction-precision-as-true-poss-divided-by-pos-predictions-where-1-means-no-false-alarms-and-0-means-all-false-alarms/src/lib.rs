//! Урок 064. Проверяем, как часто положительным прогнозам можно доверять.
//! Берём только случаи, где модель сказала «да», и считаем среди них долю верных.
//! Если из 10 срабатываний верны 8, получаем 0.8; остальные 2 — ложные тревоги.
//! Значение 1 означает отсутствие ложных тревог. Если срабатываний нет, возвращаем None.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;

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
