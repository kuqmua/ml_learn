// Урок 11.5. ROC-AUC через пары объектов.
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
    for (description, positive_scores, negative_scores, expected) in cases {
        assert!(
            !positive_scores.is_empty() && !negative_scores.is_empty(),
            "для ROC-AUC нужны оба класса"
        );
        let mut ordered_pairs = 0.0;
        for &positive in positive_scores {
            for &negative in negative_scores {
                ordered_pairs += if positive > negative {
                    1.0
                } else if positive == negative {
                    0.5
                } else {
                    0.0
                };
            }
        }
        let pair_count = (positive_scores.len() * negative_scores.len()) as f64;
        let auc = ordered_pairs / pair_count;
        assert_eq!(auc, expected);
        println!("{description}: ROC-AUC = {auc}");
    }
}
