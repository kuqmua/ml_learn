//! Вычисления и примеры урока part-059-lesson-11-precision.

/// Доля верных среди положительных прогнозов.
pub fn precision(counts: lesson_058::Counts) -> Option<f64> {
    let predicted_positives = counts.true_positives + counts.false_positives;
    if predicted_positives == 0 {
        None
    } else {
        Some(counts.true_positives as f64 / predicted_positives as f64)
    }
}
