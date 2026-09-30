// Доля верных прогнозов: число правильных ответов, делённое на общее число.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions;

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("При редком положительном классе высокая accuracy может скрыть все пропуски.");
    let counts = BinaryClassificationCounts {
        true_positives: 0,
        false_positives: 0,
        true_negatives: 9,
        false_negatives: 1,
    };
    trace_step!(counts);
    let accuracy =
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(counts);
    trace_step!(accuracy);
    assert_eq!(accuracy, Some(0.9));
    println!("Доля верных прогнозов: {accuracy:?}, хотя положительный пример пропущен.");
    let empty = BinaryClassificationCounts {
        true_positives: 0,
        false_positives: 0,
        true_negatives: 0,
        false_negatives: 0,
    };
    println!(
        "Без примеров доля не определена: {:?}",
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(empty)
    );
}
