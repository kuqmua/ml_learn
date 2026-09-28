//! Вычисления и примеры урока part-062-lesson-11-recall.

/// Доля найденных среди действительно положительных объектов.
pub fn recall(counts: lesson_058::Counts) -> Option<f64> {
    // Сохраняем результат этого шага в `actual_positives`.
    let actual_positives = counts.true_positives + counts.false_negatives;
    // Выбираем дальнейший шаг по выполнению условия.
    if actual_positives == 0 {
        // Отмечаем отсутствие подходящего значения.
        None
    // Обрабатываем случай, когда предыдущее условие не выполнено.
    } else {
        // Возвращаем присутствующее значение.
        Some(counts.true_positives as f64 / actual_positives as f64)
    }
}
