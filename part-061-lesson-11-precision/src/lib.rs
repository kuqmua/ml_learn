//! Вычисления и примеры урока part-061-lesson-11-precision.

/// Доля верных среди положительных прогнозов.
pub fn precision(counts: part_060_lesson_11_confusion_matrix::Counts) -> Option<f64> {
    // Сохраняем результат этого шага в `predicted_positives`.
    let predicted_positives = counts.true_positives + counts.false_positives;
    // Выбираем дальнейший шаг по выполнению условия.
    if predicted_positives == 0 {
        // Отмечаем отсутствие подходящего значения.
        None
    // Обрабатываем случай, когда предыдущее условие не выполнено.
    } else {
        // Возвращаем присутствующее значение.
        Some(counts.true_positives as f64 / predicted_positives as f64)
    }
}
