// Урок 11.4. Оценка F1: удвоенное произведение точности и полноты, делённое на их сумму.
// Зачем здесь эта тема: Precision и recall могут расходиться; F1 сводит их в число, чувствительное
//   к меньшему из двух.
// Почему код устроен так: Используем гармоническое среднее и рассматриваем нулевые знаменатели.
// Представь: Если precision высокий, а recall низкий, F1 не позволит одному хорошему числу скрыть
//   другое.
//
// Объединяем precision и recall из двух предыдущих уроков.
// Если обе равны нулю, формула даёт 0/0, поэтому возвращаем None.

use l066_11_calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better::calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better;

fn main() {
    for (
        _description,
        correct_pos_prediction_share_where_1_means_no_false_alarms,
        actual_pos_detection_share_where_1_means_none_missed,
        expected,
    ) in [
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        assert_eq!(
            calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum_where_1_means_no_false_alarms_or_misses_and_larger_means_better(
                Some(correct_pos_prediction_share_where_1_means_no_false_alarms),
                Some(actual_pos_detection_share_where_1_means_none_missed),
            ),
            expected
        );
    }

    plot_f1_score_as_twice_precision_times_recall_over_their_sum();
}

// Строим график по результатам урока.
fn plot_f1_score_as_twice_precision_times_recall_over_their_sum() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "F1 при correct_pos_prediction_share_where_1_means_no_false_alarms=0.8",
        "recall",
        "F1",
        &[lesson_visualization::Series {
            name: "F1",

            points: &(0..=100)
                .map(|plot_step_index| {
                    let recall_value: f64 = plot_step_index as f64 / 100.0;
                    (
                        recall_value,
                        if recall_value == 0.0 {
                            0.0
                        } else {
                            2.0 * 0.8 * recall_value / (0.8 + recall_value)
                        },
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
