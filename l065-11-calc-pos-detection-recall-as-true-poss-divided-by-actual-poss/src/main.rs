// Урок 065. Проверяем, сколько нужных случаев модель нашла.
// Берём все примеры, где правильный ответ «да», и считаем долю обнаруженных моделью.
// Если нужных случаев 10, а найдено 8, получаем 0.8: два случая пропущены.
// Значение 1 означает, что найдены все. Если нужных случаев нет, возвращаем None.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l065_11_calc_pos_detection_recall_as_true_poss_divided_by_actual_poss::calc_pos_detection_recall_as_true_poss_divided_by_actual_poss;

fn main() {
    for (
        _description,
        true_poss_as_correctly_detected_pos_cases,
        false_negs_as_missed_pos_cases,
        expected,
    ) in [
        ("найдены все", 8, 0, Some(1.0)),
        ("найдены не все", 8, 4, Some(2.0 / 3.0)),
        ("не найден ни один", 0, 4, Some(0.0)),
        ("положительных объектов нет", 0, 0, None),
    ] {
        let counts: BinaryClassificationCounts = BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases,

            false_poss_as_false_alarms_on_neg_cases: 0,

            true_negs_as_correctly_rejected_neg_cases: 0,

            false_negs_as_missed_pos_cases,
        };

        assert_eq!(
            calc_pos_detection_recall_as_true_poss_divided_by_actual_poss(counts,),
            expected
        );
    }
}
