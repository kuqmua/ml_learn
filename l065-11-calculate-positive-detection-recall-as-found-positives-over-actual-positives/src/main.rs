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
use l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives;

fn main() {
    for (_description, true_positives, false_negatives, expected) in [
        ("найдены все", 8, 0, Some(1.0)),
        ("найдены не все", 8, 4, Some(2.0 / 3.0)),
        ("не найден ни один", 0, 4, Some(0.0)),
        ("положительных объектов нет", 0, 0, None),
    ] {
        let counts: BinaryClassificationCounts = BinaryClassificationCounts {
            true_positives,

            false_positives: 0,

            true_negatives: 0,

            false_negatives,
        };

        assert_eq!(
            calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(
                counts,
            ),
            expected
        );
    }

    plot_detected_share_of_actual_positive_examples();
}

// Строим график по результатам урока.
fn plot_detected_share_of_actual_positive_examples() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Recall при фиксированном TP=2",
        "FN",
        "recall",
        &[lesson_visualization::Series {
            name: "recall",

            points: &(0..=10)
                .map(|false_negative_count| {
                    (
                        false_negative_count as f64,
                        2.0 / (2.0 + false_negative_count as f64),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
