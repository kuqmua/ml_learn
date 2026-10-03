// Урок 30.1. Поиск документов, содержащих слова запроса.
// Зачем здесь эта тема: Перед сложным поиском полезно увидеть, что даёт точное совпадение слов
//   запроса и документа.
// Почему код устроен так: Считаем общие слова на малых текстах и видим ограничение буквального
//   поиска.
// Представь: Запрос «рыжий кот» найдёт документ с теми же словами, но может пропустить текст с
//   синонимом.
//
// Что изучаем: Лексический поиск.
// Зачем это нужно: Совпадение слов запроса с документом даёт простой поиск без обученной модели.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let documents: [(&str, &str); 2] = [("rust cargo", "guide"), ("машинное обучение", "ml")];
    let query: &str = "cargo";
    for (text, _document_identifier) in documents {
        if text.split_whitespace().any(|word| word == query) {}
    }

    plot_number_of_documents_with_and_without_query_matches(documents, query);
}

// Строим график по результатам урока.
fn plot_number_of_documents_with_and_without_query_matches(
    documents: [(&str, &str); 2],
    query: &str,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Лексический поиск",
        "документов",
        &[
            (
                "найдены",
                documents
                    .iter()
                    .filter(|(text, _)| text.contains(query))
                    .count() as f64,
            ),
            (
                "не найдены",
                documents
                    .iter()
                    .filter(|(text, _)| !text.contains(query))
                    .count() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
}
