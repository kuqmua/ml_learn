// Доля верных прогнозов: число правильных ответов, делённое на общее число.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong;

fn main() {
    let counts = BinaryClassificationCounts {
        true_positives_as_correctly_detected_positive_cases: 0,
        false_positives_as_false_alarms_on_negative_cases: 0,
        true_negatives_as_correctly_rejected_negative_cases: 9,
        false_negatives_as_missed_positive_cases: 1,
    };

    assert_eq!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(counts),
        Some(0.9)
    );

    let empty = BinaryClassificationCounts {
        true_positives_as_correctly_detected_positive_cases: 0,
        false_positives_as_false_alarms_on_negative_cases: 0,
        true_negatives_as_correctly_rejected_negative_cases: 0,
        false_negatives_as_missed_positive_cases: 0,
    };
    let _ =
        &(calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(empty));
}
