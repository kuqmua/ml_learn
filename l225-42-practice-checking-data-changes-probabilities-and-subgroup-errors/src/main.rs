// Урок 225. Совместно оценивать размер подгруппы, долю верных классов и средний квадрат ошибки
// вероятностей.
// Это помогает различать ошибки решения и качества вероятностного прогноза; сдвиг входных данных
// здесь не вычисляется.

fn main() {
    #[derive(Clone, Copy, Debug)]
    struct EvaluationCase {
        group: &'static str,

        truth: bool,

        score: f64,
    }

    const EVALUATION_CASES: [EvaluationCase; 8] = [
        EvaluationCase {
            group: "A",

            truth: true,

            score: 0.9,
        },
        EvaluationCase {
            group: "A",

            truth: false,

            score: 0.1,
        },
        EvaluationCase {
            group: "A",

            truth: true,

            score: 0.8,
        },
        EvaluationCase {
            group: "A",

            truth: false,

            score: 0.2,
        },
        EvaluationCase {
            group: "B",

            truth: true,

            score: 0.4,
        },
        EvaluationCase {
            group: "B",

            truth: false,

            score: 0.7,
        },
        EvaluationCase {
            group: "B",

            truth: true,

            score: 0.6,
        },
        EvaluationCase {
            group: "B",

            truth: false,

            score: 0.3,
        },
    ];

    for group in ["A", "B"] {
        let group_cases: Vec<EvaluationCase> = EVALUATION_CASES
            .iter()
            .copied()
            .filter(|case| case.group == group)
            .collect();
        assert!(
            !group_cases.is_empty(),
            "для оценки группы нужен хотя бы один пример"
        );
        assert!(
            group_cases
                .iter()
                .all(|case| (0.0..=1.0).contains(&case.score)),
            "оценка вероятности должна быть от 0 до 1"
        );
        println!(
            "Размер группы, точность и ошибка вероятностей: {:?}",
            (
                &(group_cases.len()),
                &((|| -> f64 {
                    let data: &[EvaluationCase] = &group_cases;

                    data.iter()
                        .filter(|case| (case.score >= 0.5) == case.truth)
                        .count() as f64
                        / data.len() as f64
                })()),
                &((|| -> f64 {
                    let data: &[EvaluationCase] = &group_cases;

                    let mut squared_error_sum: f64 = 0.0;

                    for case in data {
                        squared_error_sum += (|| -> f64 {
                            let target: f64 = if case.truth { 1.0 } else { 0.0 };
                            let value: f64 = case.score - target;

                            value * value
                        })();
                    }

                    squared_error_sum / data.len() as f64
                })()),
            )
        );
    }

    let accuracy = |group| {
        let rows: Vec<_> = EVALUATION_CASES
            .iter()
            .filter(|case| case.group == group)
            .collect();
        rows.iter()
            .filter(|case| (case.score >= 0.5) == case.truth)
            .count() as f64
            / rows.len() as f64
    };
    println!(
        "Доля верных ответов: группа A={}, группа B={}",
        accuracy("A"),
        accuracy("B")
    );
    assert!(accuracy("A") > accuracy("B"));
}

// Чему учит этот урок:
// Учимся совместно оценивать размер подгруппы, долю верных классов и средний квадрат ошибки
// вероятностей.
// Это помогает различать ошибки решения и качества вероятностного прогноза; сдвиг входных данных
// здесь не вычисляется.
