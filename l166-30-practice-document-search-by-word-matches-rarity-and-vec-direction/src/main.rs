// Урок 166. Строить векторы текстов из частоты и редкости слов, сравнивать их направления и
// сортировать результаты поиска.
// Проверяем подходящий документ и запрос без известных слов; нулевые векторы не приводят к делению
// на ноль.

fn main() {
    let documents = [
        ("rust", "rust cargo память"),
        ("ml", "модель данные обучение"),
        ("math", "матрица числа строки"),
    ];
    let vocabulary: std::collections::BTreeSet<_> = documents
        .iter()
        .flat_map(|(_, text)| text.split_whitespace())
        .collect();
    let encode = |text: &str| -> Vec<f64> {
        let words: Vec<_> = text.split_whitespace().collect();
        vocabulary
            .iter()
            .map(|word| {
                let tf = words.iter().filter(|&&token| token == *word).count() as f64
                    / words.len().max(1) as f64;
                let count = documents
                    .iter()
                    .filter(|(_, doc)| doc.split_whitespace().any(|token| token == *word))
                    .count();
                tf * (((documents.len() + 1) as f64 / (count + 1) as f64).ln() + 1.0)
            })
            .collect()
    };
    for query in ["модель данные", "cargo", "пирог"] {
        let query_vec = encode(query);
        let query_len = query_vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mut found = Vec::new();
        for (id, text) in documents {
            let document_vec = encode(text);
            let document_len = document_vec.iter().map(|x| x * x).sum::<f64>().sqrt();
            let score = if query_len == 0.0 || document_len == 0.0 {
                0.0
            } else {
                query_vec
                    .iter()
                    .zip(document_vec)
                    .map(|(a, b)| a * b)
                    .sum::<f64>()
                    / (query_len * document_len)
            };
            if score > 0.0 {
                found.push((id, score));
            }
        }
        found.sort_by(|a, b| b.1.total_cmp(&a.1));
        found.truncate(2);
        println!("Запрос={query:?}, найденные документы с оценками={found:?}");
        if query == "модель данные" {
            assert_eq!(found[0].0, "ml");
        }
        if query == "пирог" {
            assert!(found.is_empty());
        }
    }
}

// Чему учит этот урок:
// Учимся строить векторы текстов из частоты и редкости слов, сравнивать их направления и
// сортировать результаты поиска.
// Проверяем подходящий документ и запрос без известных слов; нулевые векторы не приводят к делению
// на ноль.
