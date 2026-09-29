//! Урок 062. Исходы классификации: сравнение оценок с порогом и подсчёт меток.

/// Порог превращает оценки в метки перед подсчётом исходов.
pub fn count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(
// Задаём именованное поле или параметр.
    truth: &[bool],
// Задаём именованное поле или параметр.
    scores: &[f64],
// Задаём именованное поле или параметр.
    threshold: f64,
// Указываем тип возвращаемого значения.
) -> Result<l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts, &'static str>{
    // Выбираем дальнейший шаг по выполнению условия.
    if truth.len() != scores.len() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("число оценок должно совпадать с числом ответов");
    }
    // Собираем значения для `predicted` в коллекцию.
    let predicted: Vec<bool> = scores.iter().map(|&score| score >= threshold).collect();
    lesson_trace::trace_step!(predicted);
    // Используем подготовленное значение в следующем шаге примера.
    l061_11_count_correct_and_incorrect_positive_and_negative_predictions::count_binary_classification_outcomes_from_true_and_predicted_labels(truth, &predicted)
}
