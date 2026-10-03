// F1 из счётчиков: вычисление точности и полноты и их гармонического среднего.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l067_11_calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum::calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum;

fn main() {
    let counts = BinaryClassificationCounts {
        true_poss_as_correctly_detected_pos_cases: 3,
        false_poss_as_false_alarms_on_neg_cases: 1,
        true_negs_as_correctly_rejected_neg_cases: 4,
        false_negs_as_missed_pos_cases: 2,
    };

    assert!(check_f64_eq_1e_minus_12(
        calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
            counts,
        )
        .unwrap(),
        2.0 / 3.0
    ));
    let no_pos_predictions = BinaryClassificationCounts {
        true_poss_as_correctly_detected_pos_cases: 0,
        false_poss_as_false_alarms_on_neg_cases: 0,
        true_negs_as_correctly_rejected_neg_cases: 4,
        false_negs_as_missed_pos_cases: 2,
    };
    let _ = &(calc_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
            no_pos_predictions
        ));
}
