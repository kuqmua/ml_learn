// Урок 11.7. Практика: оценка прогнозов, когда один класс встречается редко.
// Связь с принятой терминологией: Метрики бинарной классификации при дисбалансе классов.
// Зачем здесь эта тема: Для несбалансированных классов accuracy, precision, recall и AUC отвечают
//   на разные вопросы.
// Почему код устроен так: Считаем их рядом на одних прогнозах, чтобы не выбирать метрику по
//   удобному числу.
// Представь: Модель «всегда отрицательно» может иметь высокую accuracy, но нулевой recall редкого
//   класса.
//
// Объединяем четыре исхода, precision, recall и F1 из уроков 11.1–11.4.
// При редком положительном классе высокая accuracy может скрывать бесполезную модель.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l062_11_count_classification_outcomes_by_comparing_scores_with_threshold::count_binary_classification_outcomes_from_targets_and_scores_at_threshold;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions;
use l064_11_calculate_positive_prediction_precision_as_true_positives_over_positive_predictions::calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions;
use l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives;
use l066_11_calculate_f1_score_as_twice_precision_times_recall_over_their_sum::calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum;

fn main() {
    let targets: [bool; 10] = [
        false, false, false, false, false, false, false, false, false, true,
    ];
    let scores: [f64; 10] = [0.1, 0.2, 0.3, 0.1, 0.2, 0.4, 0.1, 0.3, 0.2, 0.8];
    let counts: BinaryClassificationCounts =
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &targets, &scores, 0.5,
        )
        .unwrap();
    let precision: Option<f64> =
        calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(
            counts,
        );
    let recall: Option<f64> =
        calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts);
    let _harmonic_mean_score: Option<f64> =
        calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(precision, recall);

    let all_negative_scores: [f64; 10] = [0.0; 10];
    let useless: BinaryClassificationCounts =
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &targets,
            &all_negative_scores,
            0.5,
        )
        .unwrap();

    assert!(
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(useless)
            .unwrap()
            > 0.8
    );
    assert_eq!(
        calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(useless),
        Some(0.0)
    );

    plot_prediction_quality_shares_for_imbalanced_classes(
        precision,
        recall,
        calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(counts),
    );
}

// Строим график по результатам урока.
fn plot_prediction_quality_shares_for_imbalanced_classes(
    precision: core::option::Option<f64>,
    recall: core::option::Option<f64>,
    accuracy: core::option::Option<f64>,
) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Метрики при дисбалансе классов",
        "доля",
        &[
            ("precision", precision.unwrap_or(0.0)),
            ("recall", recall.unwrap_or(0.0)),
            ("accuracy", accuracy.unwrap_or(0.0)),
        ],
    )
    .expect("не удалось сохранить график");
}
