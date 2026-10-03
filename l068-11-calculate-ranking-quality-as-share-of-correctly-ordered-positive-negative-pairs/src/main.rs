// Урок 11.5. Качество ранжирования (ROC-AUC): доля правильно упорядоченных положительных и отрицательных пар.
// Связь с принятой терминологией: Площадь под ROC кривой по парам положительных и отрицательных оценок.
// Зачем здесь эта тема: Метрика при одном пороге не показывает качество ранжирования всех
//   положительных относительно отрицательных.
// Почему код устроен так: Сравниваем пары оценок разных классов: верный порядок увеличивает ROC
//   AUC.
// Представь: Если положительные примеры обычно получают оценку выше отрицательных, ранжирование
//   работает даже до выбора порога.
//
// Значение 1 означает, что каждая положительная оценка выше отрицательной; 0 — наоборот.
// При равных оценках даём паре половину балла, как в стандартном определении ROC-AUC.

fn main() {
    let cases: [(&str, &[f64], &[f64], f64); 4] = [
        ("идеальный порядок", &[0.9, 0.7], &[0.6, 0.2], 1.0),
        ("обратный порядок", &[0.1, 0.2], &[0.8, 0.9], 0.0),
        ("одинаковые оценки", &[0.5, 0.5], &[0.5, 0.5], 0.5),
        ("смешанный порядок", &[0.8, 0.2], &[0.6, 0.4], 0.5),
    ];
    for (_description, pos_scores, neg_scores, expected) in cases {
        assert!(
            !pos_scores.is_empty() && !neg_scores.is_empty(),
            "для ROC-AUC нужны оба класса"
        );
        let mut ordered_pairs: f64 = 0.0;
        for &pos in pos_scores {
            for &neg in neg_scores {
                ordered_pairs += if pos > neg {
                    1.0
                } else if pos == neg {
                    0.5
                } else {
                    0.0
                };
            }
        }
        let pair_count: f64 = (pos_scores.len() * neg_scores.len()) as f64;
        let roc_auc_where_1_means_correct_order_0_means_reversed_order_and_half_means_no_pairwise_ranking_advantage: f64 = ordered_pairs / pair_count;
        assert_eq!(roc_auc_where_1_means_correct_order_0_means_reversed_order_and_half_means_no_pairwise_ranking_advantage, expected);
    }

    plot_detected_pos_share_against_false_pos_share();
}

// Строим график по результатам урока.
fn plot_detected_pos_share_against_false_pos_share() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "ROC-кривая: идеальное ранжирование",
        "доля ложных срабатываний",
        "полнота",
        &[
            lesson_visualization::Series {
                name: "идеал",

                points: &[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)].to_vec(),
            },
            lesson_visualization::Series {
                name: "случайный порядок",

                points: &[(0.0, 0.0), (1.0, 1.0)].to_vec(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
