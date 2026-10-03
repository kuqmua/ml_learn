// Урок 42.5. Практика: проверка изменений данных, вероятностей и ошибок по подгруппам.
// Зачем здесь эта тема: После выпуска одновременно важны сдвиг входов, калибровка, качество групп и
//   неопределённость.
// Почему код устроен так: Собираем метрики вместе и интерпретируем их с учётом числа наблюдений.
// Представь: Хорошее среднее качество ещё не отменяет сдвиг входов или ошибки отдельной группы.
//
// Что повторяем вместе: сдвиг распределения, калибровка, ошибки подгрупп, ограничения модели.
// Зачем это нужно: Качество AI нужно смотреть по подгруппам и вероятностям, учитывая ограничения маленькой
//   выборки.
// Что показывает программа: Разделяем набор по подгруппам. Для каждой группы считаем accuracy и
//   вероятностную ошибку Brier. Указываем ограничение вывода из очень маленькой выборки.
// Что проверить при изменении примера: Покажи ошибки, интервалы неопределённости и конкретные ограничения
//   вывода.
// Дополнительная практика: Составь набор сложных случаев для одного классификатора и отчёт по подгруппам.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

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
        let _ = (
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
        );
    }

    plot_correct_prediction_share_in_each_evaluation_subgroup();

    fn plot_correct_prediction_share_in_each_evaluation_subgroup() {
        let group_correct_prediction_share: &dyn Fn(&str) -> f64 = &|group: &str| {
            let cases: Vec<&EvaluationCase> = EVALUATION_CASES
                .iter()
                .filter(|case| case.group == group)
                .collect();
            cases
                .iter()
                .filter(|case| (case.score >= 0.5) == case.truth)
                .count() as f64
                / cases.len() as f64
        };
        lesson_visualization::bar_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Accuracy по подгруппам",
            "accuracy",
            &[
                ("A", group_correct_prediction_share("A")),
                ("B", group_correct_prediction_share("B")),
            ],
        )
        .expect("не удалось сохранить график");
    }
}
