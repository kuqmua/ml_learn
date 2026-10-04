// Урок 242. Сравниваем способы определения темы текста на одинаковых примерах.
// Один простой способ всегда выбирает наиболее частую тему; другой учитывает слова текста.
// Проверяем ответы на данных, которые не использовали для подготовки решения.
// Разбираем конкретные ошибки: больше сложностей в программе ещё не означает больше правильных ответов.

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
    let _ = (baseline, correct);
}

// Чему учит этот урок:
// Учимся выбирать тему текста по совпадениям слов с обучающими примерами и считать точность на
// отдельном тесте.
// Для каждого текста получаем тему и число совпадений; отдельно считаем долю класса code в
// обучении.
// Полного сравнения нескольких методов на тесте и отчёта о конкретных ошибках текущий код пока не
// делает.
