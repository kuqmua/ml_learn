// Урок 242. Сравнивать постоянный класс и поиск по словам на одном и том же тестовом наборе.
// На отдельных сложных текстах разбираем конкретную ошибку: без знакомых слов выбор при ничьей
// может дать неверную тему.

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

    let baseline: f64 = TRAINING_EXAMPLES
        .iter()
        .filter(|(_, expected_topic)| *expected_topic == "code")
        .count() as f64
        / TRAINING_EXAMPLES.len() as f64;

    // Выполняем вычисления из примера.
    let _ = (&baseline, &correct);

    let majority =
        if TRAINING_EXAMPLES.iter().filter(|v| v.1 == "ml").count() * 2 > TRAINING_EXAMPLES.len() {
            "ml"
        } else {
            "code"
        };
    let baseline_correct = TEST_EXAMPLES.iter().filter(|v| v.1 == majority).count();
    println!(
        "На одном тесте из {} текстов: постоянный класс верно={baseline_correct}, поиск по словам верно={correct}",
        TEST_EXAMPLES.len()
    );
    assert!(correct > baseline_correct);
    // Отдельный сложный набор показывает ограничение буквального совпадения.
    for (text, target) in [("сбой сборки", "code"), ("нейросетевая оптимизация", "ml")]
    {
        let (prediction, matches) =
            choose_topic_and_count_matching_training_words(text, &TRAINING_EXAMPLES);
        println!("Текст={text:?}, ожидаем={target}, получено={prediction}, совпадений={matches}");
        if prediction != target {
            println!("Ошибка: знакомого слова нет; при ничьей правило выбрало code.");
        }
    }
    assert_ne!(
        choose_topic_and_count_matching_training_words(
            "нейросетевая оптимизация",
            &TRAINING_EXAMPLES
        )
        .0,
        "ml"
    );
}

// Чему учит этот урок:
// Учимся сравнивать постоянный класс и поиск по словам на одном и том же тестовом наборе.
// На отдельных сложных текстах разбираем конкретную ошибку: без знакомых слов выбор при ничьей
// может дать неверную тему.
