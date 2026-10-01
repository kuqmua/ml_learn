// Доля верных прогнозов: число правильных ответов, делённое на общее число.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions;

fn main() {
    let counts = BinaryClassificationCounts {
        true_positives: 0,
        false_positives: 0,
        true_negatives: 9,
        false_negatives: 1,
    };

    assert_eq!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(counts),
        Some(0.9)
    );

    let empty = BinaryClassificationCounts {
        true_positives: 0,
        false_positives: 0,
        true_negatives: 0,
        false_negatives: 0,
    };
    let _ =
        &(calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(empty));
}
