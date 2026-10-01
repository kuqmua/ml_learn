//! Урок 062. Исходы классификации: сравнение оценок с порогом и подсчёт меток.

/// Порог превращает оценки в метки перед подсчётом исходов.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::{
    BinaryClassificationCounts, count_binary_classification_outcomes_from_targets_and_predictions,
};

pub fn count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
    truth: &[bool],

    scores: &[f64],

    threshold: f64,
) -> Result<BinaryClassificationCounts, &'static str> {
    if truth.len() != scores.len() {
        return Err("число оценок должно совпадать с числом ответов");
    }

    count_binary_classification_outcomes_from_targets_and_predictions(
        truth,
        &scores
            .iter()
            .map(|&score| score >= threshold)
            .collect::<Vec<_>>(),
    )
}
