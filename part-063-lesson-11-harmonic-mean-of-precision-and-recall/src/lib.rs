//! Вычисления и примеры урока part-063-lesson-11-harmonic-mean-of-precision-and-recall.

/// Гармоническое среднее precision и recall.
pub fn harmonic_mean_of_precision_and_recall(
    precision: Option<f64>,
    recall: Option<f64>,
) -> Option<f64> {
    // Разбираем результат по его возможным вариантам.
    match (precision, recall) {
        // Выполняем действие для этого варианта данных.
        (Some(p), Some(r)) if p + r > 0.0 => Some(2.0 * p * r / (p + r)),
        // Выполняем действие для этого варианта данных.
        _ => None,
    }
}

/// Получаем обе метрики из счётчиков предыдущих уроков.
pub fn harmonic_mean_of_precision_and_recall_from_counts(
    counts: part_060_lesson_11_confusion_matrix::Counts,
) -> Option<f64> {
    // Используем подготовленное значение в следующем шаге примера.
    harmonic_mean_of_precision_and_recall(
        part_061_lesson_11_precision::precision(counts),
        part_062_lesson_11_recall::recall(counts),
    )
}
