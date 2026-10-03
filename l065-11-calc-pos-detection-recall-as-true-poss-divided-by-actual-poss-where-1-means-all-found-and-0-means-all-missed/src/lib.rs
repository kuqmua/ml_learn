//! Урок 065. Проверяем, сколько нужных случаев модель нашла.
//! Берём все примеры, где правильный ответ «да», и считаем долю обнаруженных моделью.
//! Если нужных случаев 10, а найдено 8, получаем 0.8: два случая пропущены.
//! Значение 1 означает, что найдены все. Если нужных случаев нет, возвращаем None.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;

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
