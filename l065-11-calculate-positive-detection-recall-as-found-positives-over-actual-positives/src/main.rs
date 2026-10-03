// Урок 11.3. Полнота обнаружения: доля найденных среди всех действительно положительных примеров.
// Связь с принятой терминологией: Полнота положительного класса по счётчикам бинарной классификации.
// Зачем здесь эта тема: Когда пропуск события дорог, важно знать, какую долю истинно положительных
//   нашли.
// Почему код устроен так: Делим TP на TP+FN; знаменатель здесь зависит от истинных меток, а не
//   прогнозов.
// Представь: Из 10 реальных положительных случаев нашли 7: recall равен 7/10, независимо от ложных
//   тревог.
//
// Используем те же четыре счётчика. Если положительных объектов нет,
// значение здесь считаем неопределённым.

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed;

fn main() {
    for (
        _description,
        true_poss_as_correctly_detected_pos_cases,
        false_negs_as_missed_pos_cases,
        expected,
    ) in [
        ("найдены все", 8, 0, Some(1.0)),
        ("найдены не все", 8, 4, Some(2.0 / 3.0)),
        ("не найден ни один", 0, 4, Some(0.0)),
        ("положительных объектов нет", 0, 0, None),
    ] {
        let counts: BinaryClassificationCounts = BinaryClassificationCounts {
            true_poss_as_correctly_detected_pos_cases,

            false_poss_as_false_alarms_on_neg_cases: 0,

            true_negs_as_correctly_rejected_neg_cases: 0,

            false_negs_as_missed_pos_cases,
        };

        assert_eq!(
            calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed(
                counts,
            ),
            expected
        );
    }

    plot_detected_share_of_actual_pos_examples();
}

// Строим график по результатам урока.
fn plot_detected_share_of_actual_pos_examples() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Recall при фиксированном TP=2",
        "FN",
        "recall",
        &[lesson_visualization::Series {
            name: "recall",

            points: &(0..=10)
                .map(|false_neg_count| {
                    (false_neg_count as f64, 2.0 / (2.0 + false_neg_count as f64))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
