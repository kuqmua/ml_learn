// F1 из счётчиков: вычисление точности и полноты и их гармонического среднего.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l067_11_calculate_f1_from_counts_by_combining_precision_and_recall::calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better;

fn main() {
    let counts = BinaryClassificationCounts {
        true_positives_as_correctly_detected_positive_cases: 3,
        false_positives_as_false_alarms_on_negative_cases: 1,
        true_negatives_as_correctly_rejected_negative_cases: 4,
        false_negatives_as_missed_positive_cases: 2,
    };

    assert!(check_f64_eq_1e_minus_12(calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(
            counts,
        )
        .unwrap(), 2.0 / 3.0));
    let no_positive_predictions = BinaryClassificationCounts {
        true_positives_as_correctly_detected_positive_cases: 0,
        false_positives_as_false_alarms_on_negative_cases: 0,
        true_negatives_as_correctly_rejected_negative_cases: 4,
        false_negatives_as_missed_positive_cases: 2,
    };
    let _ = &(calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(
            no_positive_predictions
        ));
}
