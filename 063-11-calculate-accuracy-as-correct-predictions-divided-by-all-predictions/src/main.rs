// Доля верных прогнозов: число правильных ответов, делённое на общее число.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts as Counts;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions as operation;

fn main() {
    lesson_trace::enable();
    // При редком положительном классе высокая accuracy может скрыть все пропуски.
    let counts = Counts {
        true_positives: 0,
        false_positives: 0,
        true_negatives: 9,
        false_negatives: 1,
    };
    lesson_trace::trace_step!(counts);
    let accuracy = operation(counts);
    lesson_trace::trace_step!(accuracy);
    assert_eq!(accuracy, Some(0.9));
    println!("Доля верных прогнозов: {accuracy:?}, хотя положительный пример пропущен.");
    let empty = Counts {
        true_positives: 0,
        false_positives: 0,
        true_negatives: 0,
        false_negatives: 0,
    };
    println!("Без примеров доля не определена: {:?}", operation(empty));
}
