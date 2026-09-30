// Исходы классификации: сравнение оценок с порогом и подсчёт меток.

use l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold;

fn main() {
    let truth = [true, false, true, false];
    let scores = [0.9, 0.6, 0.5, 0.1];
    for threshold in [0.5, 0.7] {
        let _counts =
            count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(
                &truth, &scores, threshold,
            )
            .unwrap();
    }
}
