// Урок 208. Искать совпадения запроса в локальных документах и возвращать текст вместе с источником.
// Если совпадений нет, выдаём отказ; это простой поиск по словам, а не проверка всех утверждений
// документа.

fn main() {
    const DOCUMENTS: [(&str, &str); 3] = [
        (
            "rust.md",
            "Rust использует владение для управления памятью.",
        ),
        (
            "ml.md",
            "Модель обучают на примерах и проверяют на отложенных данных.",
        ),
        ("math.md", "Градиент указывает направление роста функции."),
    ];
    fn answer_from_matching_document_or_abstain(question: &str) -> String {
        let query_terms: Vec<String> = question
            .to_lowercase()
            .split_whitespace()
            .map(|raw_word| {
                raw_word
                    .trim_matches(|character: char| !character.is_alphabetic())
                    .to_string()
            })
            .filter(|raw_word| raw_word.len() > 3)
            .collect();
        let best_match: (usize, &str, &str) = DOCUMENTS
            .iter()
            .map(|&(source_identifier, document)| {
                let lower: String = document.to_lowercase();

                (
                    query_terms
                        .iter()
                        .filter(|term| lower.contains(term.as_str()))
                        .count(),
                    source_identifier,
                    document,
                )
            })
            .max_by_key(|candidate| candidate.0)
            .unwrap();
        if best_match.0 == 0 {
            "Нет подтверждения в локальных документах.".into()
        } else {
            format!("{} [источник: {}]", best_match.2, best_match.1)
        }
    }

    let _ = &(answer_from_matching_document_or_abstain("Как проверяют модель на данных?"));
    let _ = &(answer_from_matching_document_or_abstain("Где рецепт пирога?"));

    let supported = answer_from_matching_document_or_abstain("Как проверяют модель на данных?");
    let missing = answer_from_matching_document_or_abstain("Где рецепт пирога?");
    println!(
        "С совпадением: {supported}
Без совпадения: {missing}"
    );
    assert!(supported.contains("ml.md"));
    assert_eq!(missing, "Нет подтверждения в локальных документах.");
}

// Чему учит этот урок:
// Учимся искать совпадения запроса в локальных документах и возвращать текст вместе с источником.
// Если совпадений нет, выдаём отказ; это простой поиск по словам, а не проверка всех утверждений
// документа.
