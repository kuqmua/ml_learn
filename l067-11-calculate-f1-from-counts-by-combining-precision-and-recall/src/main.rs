// F1 из счётчиков: вычисление точности и полноты и их гармонического среднего.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l067_11_calculate_f1_from_counts_by_combining_precision_and_recall::calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum;

fn main() {
    let counts = BinaryClassificationCounts {
        true_positives: 3,
        false_positives: 1,
        true_negatives: 4,
        false_negatives: 2,
    };

    assert!((calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
            counts,
        )
        .unwrap() - 2.0 / 3.0).abs() < 1e-12);
    let no_positive_predictions = BinaryClassificationCounts {
        true_positives: 0,
        false_positives: 0,
        true_negatives: 4,
        false_negatives: 2,
    };
    let _ = &(calculate_f1_score_from_counts_by_multiplying_precision_and_recall_by_two_then_dividing_by_sum(
            no_positive_predictions
        ));
}
