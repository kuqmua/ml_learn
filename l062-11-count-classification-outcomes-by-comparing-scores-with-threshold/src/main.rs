// Исходы классификации: сравнение оценок с порогом и подсчёт меток.

use l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold;

use lesson_trace::{enable, trace_step};

fn main() {
    enable();
    let truth = [true, false, true, false];
    let scores = [0.9, 0.6, 0.5, 0.1];
    trace_step!(truth);
    trace_step!(scores);
    for threshold in [0.5, 0.7] {
        trace_step!(threshold);
        let counts = count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(
            &truth, &scores, threshold,
        )
        .unwrap();
        trace_step!(counts);
        println!("Порог {threshold}: {counts:?}");
    }
    println!("Оценка, равная порогу, относится к положительному классу.");
}
