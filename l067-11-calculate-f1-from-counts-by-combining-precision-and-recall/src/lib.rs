//! Урок 067. F1 из счётчиков: вычисление точности и полноты и их гармонического среднего.

/// Получаем обе метрики из счётчиков предыдущих уроков.
/// F1 из счётчиков: находим precision и recall, затем делим их удвоенное произведение на сумму.
use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l064_11_calculate_positive_prediction_precision_as_true_positives_over_positive_predictions::calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions;
use l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives;
use l066_11_calculate_f1_score_as_twice_precision_times_recall_over_their_sum::calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum;

use lesson_trace::trace_note;

pub fn calculate_f1_score_from_counts_by_combining_precision_and_recall_as_twice_product_over_sum(
    counts: BinaryClassificationCounts,
) -> Option<f64> {
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
        calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(
            counts,
        ),
        calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts),
    )
}
