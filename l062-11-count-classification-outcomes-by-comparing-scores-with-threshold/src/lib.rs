//! Урок 062. Исходы классификации: сравнение оценок с порогом и подсчёт меток.

/// Порог превращает оценки в метки перед подсчётом исходов.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::{
    BinaryClassificationCounts, count_binary_classification_outcomes_from_true_and_predicted_labels,
};

pub fn count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(
    truth: &[bool],

    scores: &[f64],

    threshold: f64,
) -> Result<BinaryClassificationCounts, &'static str> {
    if truth.len() != scores.len() {
        return Err("число оценок должно совпадать с числом ответов");
    }
    let predicted: Vec<bool> = scores.iter().map(|&score| score >= threshold).collect();
    count_binary_classification_outcomes_from_true_and_predicted_labels(truth, &predicted)
}
