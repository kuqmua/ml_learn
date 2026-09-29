//! Урок 063. Удвоенное произведение точности положительных прогнозов и полноты, делённое на их сумму.

/// Гармоническое среднее precision и recall.
/// F1 — гармоническое среднее precision и recall: 2·precision·recall / (precision + recall).
pub fn calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
    precision: Option<f64>,
    recall: Option<f64>,
) -> Option<f64> {
    // Разбираем результат по его возможным вариантам.
    match (precision, recall) {
        // Коэффициент 2 делает 2pr/(p+r) гармоническим средним двух метрик.
        (Some(p), Some(r)) if p + r > 0.0 => Some(2.0 * p * r / (p + r)),
        // Выполняем действие для этого варианта данных.
        _ => None,
    }
}

/// Получаем обе метрики из счётчиков предыдущих уроков.
/// F1 из счётчиков: находим precision и recall, затем делим их удвоенное произведение на сумму.
pub fn calculate_f1_score_from_counts_by_combining_precision_and_recall_as_twice_product_over_sum(
    counts: part_060_lesson_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts,
) -> Option<f64> {
    // Используем подготовленное значение в следующем шаге примера.
    calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
        part_061_lesson_11_divide_true_positives_by_all_positive_predictions::calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(counts),
        part_062_lesson_11_divide_detected_positives_by_all_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts),
    )
}
