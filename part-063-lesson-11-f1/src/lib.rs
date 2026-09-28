//! Вычисления и примеры урока part-063-lesson-11-f1.

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
    counts: lesson_058::Counts,
) -> Option<f64> {
    // Используем подготовленное значение в следующем шаге примера.
    harmonic_mean_of_precision_and_recall(lesson_059::precision(counts), lesson_060::recall(counts))
}
