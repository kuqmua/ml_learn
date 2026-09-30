//! Урок 066. Оценка F1: удвоенное произведение точности и полноты, делённое на их сумму.

/// Гармоническое среднее precision и recall.
/// F1 — гармоническое среднее precision и recall: 2·precision·recall / (precision + recall).
pub fn calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
    precision: Option<f64>,
    recall: Option<f64>,
) -> Option<f64> {
    lesson_trace::trace_note!("Разбираем результат по его возможным вариантам.");
    lesson_trace::trace_note!("Коэффициент 2 делает 2pr/(p+r) гармоническим средним двух метрик.");
    lesson_trace::trace_note!("Выполняем действие для этого варианта данных.");
    match (precision, recall) {
        (Some(p), Some(r)) if p + r > 0.0 => Some(2.0 * p * r / (p + r)),

        _ => None,
    }
}
