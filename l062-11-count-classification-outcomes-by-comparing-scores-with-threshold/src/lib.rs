//! Урок 062. Исходы классификации: сравнение оценок с порогом и подсчёт меток.

/// Порог превращает оценки в метки перед подсчётом исходов.
use lesson_trace::{trace_note, trace_step};

pub fn count_binary_classification_outcomes_from_true_labels_and_scores_at_threshold(

    truth: &[bool],

    scores: &[f64],

    threshold: f64,

) -> Result<l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts, &'static str>{
    trace_note!("Задаём именованное поле или параметр.");
    trace_note!("Задаём именованное поле или параметр.");
    trace_note!("Задаём именованное поле или параметр.");
    trace_note!("Указываем тип возвращаемого значения.");
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if truth.len() != scores.len() {
        trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("число оценок должно совпадать с числом ответов");
    }
    trace_note!("Собираем значения для `predicted` в коллекцию.");
    let predicted: Vec<bool> = scores.iter().map(|&score| score >= threshold).collect();
    trace_step!(predicted);
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    l061_11_count_correct_and_incorrect_positive_and_negative_predictions::count_binary_classification_outcomes_from_true_and_predicted_labels(truth, &predicted)
}
