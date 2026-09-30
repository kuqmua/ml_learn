// Урок 47.1. Итоговый проект: сравнение способов решения задачи и разбор ошибок.
// Связь с принятой терминологией: Итоговый AI проект со сравнением подходов и анализом ошибок.
// Зачем здесь эта тема: Итоговый AI-проект требует сравнивать подход с baseline и разбирать ошибки
//   на данных вне обучения.
// Почему код устроен так: Сравниваем простую базовую модель с более сложным методом на общих
//   примерах и разбираем ошибки.
// Представь: Если простой прогноз по большинству классов не хуже текстового классификатора,
//   сложность пока не оправдана.
//
// Что применяем: самостоятельный дизайн, сравнение подходов, анализ ошибок, презентация результата.
// Зачем это нужно: Итоговый проект соединяет простую текстовую модель с baseline, отдельной оценкой и
//   разбором прогнозов.
// Что показывает программа: Считаем частоту большинства как простую исходную точку. Оцениваем классификатор
//   на фразах вне обучения. Разбираем каждый прогноз вместе с истинной темой и числом совпавших слов.
// Что проверить при изменении примера: Из чистого checkout выполняются подготовка данных, обучение, оценка
//   и инференс; выводы подтверждены метриками.
// Возможное расширение: Сделай прикладной проект: классификатор, поиск по документам или небольшой
//   генератор; обоснуй выбор и оформи README.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    const TRAINING_EXAMPLES: [(&str, &str); 6] = [
        ("ошибка компиляции rust", "code"),
        ("cargo не собирает проект", "code"),
        ("заимствование и владение", "code"),
        ("градиент модели", "ml"),
        ("обучение нейросети", "ml"),
        ("признаки и метрики", "ml"),
    ];

    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для baseline нужны обучающие примеры"
    );
    let baseline: f64 = TRAINING_EXAMPLES
        .iter()
        .filter(|(_, expected_topic)| *expected_topic == "code")
        .count() as f64
        / TRAINING_EXAMPLES.len() as f64;
    const TEST_EXAMPLES: [(&str, &str); 2] = [("ошибка cargo", "code"), ("метрики модели", "ml")];
    assert!(
        !TEST_EXAMPLES.is_empty(),
        "для оценки нужны тестовые примеры"
    );

    fn collect_unique_words_from_text(sample_text: &str) -> std::collections::BTreeSet<&str> {
        sample_text.split_whitespace().collect()
    }
    fn choose_topic_and_count_matching_training_words(
        query: &str,

        training_examples: &[(&str, &str)],
    ) -> (&'static str, usize) {
        let query_text_units: std::collections::BTreeSet<&str> =
            collect_unique_words_from_text(query);
        let mut scores: std::collections::BTreeMap<&str, usize> =
            std::collections::BTreeMap::from([("code", 0usize), ("ml", 0)]);
        for &(sample_text, target) in training_examples {
            *scores.get_mut(target).unwrap() += query_text_units
                .intersection(&collect_unique_words_from_text(sample_text))
                .count();
        }
        let code_score: usize = scores["code"];
        let machine_learning_score: usize = scores["ml"];
        (
            if machine_learning_score > code_score {
                "ml"
            } else {
                "code"
            },
            if code_score > machine_learning_score {
                code_score
            } else {
                machine_learning_score
            },
        )
    }

    let correct: usize = TEST_EXAMPLES
        .iter()
        .filter(|&&(query_text, expected_topic)| {
            choose_topic_and_count_matching_training_words(query_text, &TRAINING_EXAMPLES).0
                == expected_topic
        })
        .count();
    let _ = &(correct as f64 / TEST_EXAMPLES.len() as f64);
    for &(sample_text, _expected_topic) in &TEST_EXAMPLES {
        let (_predicted_topic, _overlap_count): (&str, usize) =
            choose_topic_and_count_matching_training_words(sample_text, &TRAINING_EXAMPLES);
    }

    plot_correct_prediction_shares_for_compared_text_classifiers(baseline, correct);

    fn plot_correct_prediction_shares_for_compared_text_classifiers(baseline: f64, correct: usize) {
        let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Итоговый AI-проект",
            "accuracy",
            &[
                ("baseline", baseline),
                ("test", correct as f64 / TEST_EXAMPLES.len() as f64),
            ],
        )
        .expect("не удалось сохранить график");
    }
}
