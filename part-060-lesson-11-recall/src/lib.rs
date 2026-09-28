//! Вычисления и примеры урока part-060-lesson-11-recall.

/// Доля найденных среди действительно положительных объектов.
pub fn recall(counts: lesson_058::Counts) -> Option<f64> {
    let actual_positives = counts.true_positives + counts.false_negatives;
    if actual_positives == 0 {
        None
    } else {
        Some(counts.true_positives as f64 / actual_positives as f64)
    }
}
