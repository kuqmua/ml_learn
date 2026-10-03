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

use l061_11_count_binary_classification_outcomes_from_targets_and_predictions::BinaryClassificationCounts;
use l062_11_count_binary_classification_outcomes_from_targets_and_scores_at_threshold::count_binary_classification_outcomes_from_targets_and_scores_at_threshold;
use l063_11_calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong::calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong;
use l064_11_calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions_where_1_means_no_false_alarms_and_0_means_all_false_alarms::calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions_where_1_means_no_false_alarms_and_0_means_all_false_alarms;
use l065_11_calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed::calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed;
use l066_11_calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better::calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better;

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
    let correct_pos_prediction_share_where_1_means_no_false_alarms: Option<f64> =
        calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions_where_1_means_no_false_alarms_and_0_means_all_false_alarms(
            counts,
        );
    let actual_pos_detection_share_where_1_means_none_missed: Option<f64> =
        calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed(counts);
    let _: Option<f64> =
        calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(correct_pos_prediction_share_where_1_means_no_false_alarms, actual_pos_detection_share_where_1_means_none_missed);

    let all_neg_scores: [f64; 10] = [0.0; 10];
    let useless: BinaryClassificationCounts =
        count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &targets,
            &all_neg_scores,
            0.5,
        )
        .unwrap();

    assert!(
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(useless)
            .unwrap()
            > 0.8
    );
    assert_eq!(
        calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed(useless),
        Some(0.0)
    );

    plot_prediction_quality_shares_for_imbalanced_classes(
        correct_pos_prediction_share_where_1_means_no_false_alarms,
        actual_pos_detection_share_where_1_means_none_missed,
        calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(counts),
    );
}

// Строим график по результатам урока.
fn plot_prediction_quality_shares_for_imbalanced_classes(
    correct_pos_prediction_share_where_1_means_no_false_alarms: core::option::Option<f64>,
    actual_pos_detection_share_where_1_means_none_missed: core::option::Option<f64>,
    accuracy: core::option::Option<f64>,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Метрики при дисбалансе классов",
        "доля",
        &[
            (
                "precision",
                correct_pos_prediction_share_where_1_means_no_false_alarms.unwrap_or(0.0),
            ),
            (
                "recall",
                actual_pos_detection_share_where_1_means_none_missed.unwrap_or(0.0),
            ),
            ("accuracy", accuracy.unwrap_or(0.0)),
        ],
    )
    .expect("не удалось сохранить график");
}
