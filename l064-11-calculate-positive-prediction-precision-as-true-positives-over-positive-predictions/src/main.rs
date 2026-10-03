// Урок 11.2. Точность положительных прогнозов: доля верных среди всех положительных прогнозов.
// Связь с принятой терминологией: Точность положительных прогнозов по счётчикам бинарной классификации.
// Зачем здесь эта тема: Когда ложные тревоги дороги, важно знать долю верных среди положительных
//   прогнозов.
// Почему код устроен так: Делим TP на TP+FP и отдельно обрабатываем отсутствие положительных
//   прогнозов.
// Представь: Из 10 положительных прогнозов 7 верных: precision равен 7/10, независимо от
//   пропущенных случаев.
//
// Используем четыре счётчика из урока 11.1. Если положительных прогнозов нет,
// значение здесь считаем неопределённым.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l064_11_calculate_positive_prediction_precision_as_true_positives_over_positive_predictions::calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions_where_1_means_no_false_alarms_and_0_means_all_false_alarms;

fn main() {
    for (
        _description,
        true_poss_as_correctly_detected_pos_cases,
        false_poss_as_false_alarms_on_neg_cases,
        expected,
    ) in [
        ("все положительные прогнозы верны", 8, 0, Some(1.0)),
        ("часть прогнозов ошибочна", 8, 2, Some(0.8)),
        ("все положительные прогнозы ошибочны", 0, 2, Some(0.0)),
        ("положительных прогнозов нет", 0, 0, None),
    ] {
        let counts: BinaryClassificationCounts = BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases,

            false_poss_as_false_alarms_on_neg_cases,

            true_negs_as_correctly_rejected_neg_cases: 0,

            false_negs_as_missed_pos_cases: 0,
        };

        assert_eq!(calc_pos_prediction_precision_as_true_poss_divided_by_pos_predictions_where_1_means_no_false_alarms_and_0_means_all_false_alarms(counts), expected);
    }

    plot_true_pos_share_among_pos_predictions();
}

// Строим график по результатам урока.
fn plot_true_pos_share_among_pos_predictions() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Precision при фиксированном TP=2",
        "FP",
        "precision",
        &[lesson_visualization::Series {
            name: "precision",

            points: &(0..=10)
                .map(|false_pos_count| {
                    (false_pos_count as f64, 2.0 / (2.0 + false_pos_count as f64))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
