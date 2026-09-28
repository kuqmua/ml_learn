//! Вычисления и примеры урока part-063-lesson-11-f1.

/// Гармоническое среднее precision и recall.
pub fn f1(precision: Option<f64>, recall: Option<f64>) -> Option<f64> {
    // Разбираем результат по его возможным вариантам.
    match (precision, recall) {
        // Выполняем действие для этого варианта данных.
        (Some(p), Some(r)) if p + r > 0.0 => Some(2.0 * p * r / (p + r)),
        // Выполняем действие для этого варианта данных.
        _ => None,
    }
}

/// Получаем обе метрики из счётчиков предыдущих уроков.
pub fn f1_from_counts(counts: lesson_058::Counts) -> Option<f64> {
    // Используем подготовленное значение в следующем шаге примера.
    f1(lesson_059::precision(counts), lesson_060::recall(counts))
}
