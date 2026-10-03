// Урок 064. Проверяем, как часто положительным прогнозам можно доверять.
// Берём только случаи, где модель сказала «да», и считаем среди них долю верных.
// Если из 10 срабатываний верны 8, получаем 0.8; остальные 2 — ложные тревоги.
// Значение 1 означает отсутствие ложных тревог. Если срабатываний нет, возвращаем None.

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l064_11_calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions::calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions;

fn main() {
    for (
        _description,
        true_poss_as_correctly_detected_pos_cases,
        false_poss_as_false_alarms_on_neg_cases,
        expected,
    ) in [
        ("все положительные прогнозы верны", 8, 0, Some(1.0)),
        ("часть прогнозов ошибочна", 8, 2, Some(0.8)),
        ("все положительные прогнозы ошибочны", 0, 2, Some(0.0)),
        ("положительных прогнозов нет", 0, 0, None),
    ] {
        let counts: BinaryClassificationCounts = BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases,

            false_poss_as_false_alarms_on_neg_cases,

            true_negs_as_correctly_rejected_neg_cases: 0,

            false_negs_as_missed_pos_cases: 0,
        };

        assert_eq!(
            calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions(counts),
            expected
        );
    }
}
