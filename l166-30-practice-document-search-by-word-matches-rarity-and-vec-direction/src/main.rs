// Урок 30.5. Практика: поиск документов по совпадениям слов, их редкости и направлениям векторов.
// Зачем здесь эта тема: Поиск соединяет токенизацию, вес слов, нормировку и выбор лучших
//   документов.
// Почему код устроен так: Проводим один запрос через весь конвейер и показываем вклад каждой стадии
//   в порядок результатов.
// Представь: Запрос сначала превращается в слова, затем в веса, затем получает оценку для каждого
//   документа.
//
// Что повторяем вместе: лексический поиск, TF-IDF, сходство направлений независимо от длин, top-k.
// Зачем это нужно: Поиск сопоставляет запрос с документами и учитывает не только частоту слова, но и его
//   редкость в коллекции.
// Что показывает программа: Токенизируем запрос, рассчитываем TF-IDF по документам и возвращаем top-k
//   источников.
// Что проверить при изменении примера: Проверь запрос без совпадений, повторные слова и отсутствие
//   документов из test в индексе оценки.
// Дополнительная практика: Индексируй локальный набор документов и возвращай top-k с оценками и источниками.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    const DOCUMENTS: [(&str, &str); 3] = [
        ("rust", "Rust управляет памятью без сборщика мусора"),
        ("ml", "Модель учится по обучающим данным"),
        ("math", "Матрица хранит числа в строках и столбцах"),
    ];

    fn split_text_into_lowercase_words(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(str::to_string)
            .collect()
    }

    // Выполняем вычисления из примера.
    let _ = (|| -> Vec<(&'static str, f64)> {
        let query: &str = "модель данные";
        let query_text_units: Vec<String> = split_text_into_lowercase_words(query);
        let mut ranked_results: Vec<(&str, f64)> = vec![];
        for &(document_identifier, document_text) in &DOCUMENTS {
            let document_text_units: Vec<String> = split_text_into_lowercase_words(document_text);
            let mut text_unit_counts: std::collections::BTreeMap<&String, usize> =
                std::collections::BTreeMap::new();
            for text_unit in &document_text_units {
                *text_unit_counts.entry(text_unit).or_insert(0usize) += 1;
            }
            let mut relevance_score: f64 = 0.0;
            for word in query_text_units
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
            {
                let inverse_document_frequency_as_word_rarity_weight: f64 = (|| -> f64 {
                    let value: f64 = (DOCUMENTS.len() + 1) as f64
                        / (DOCUMENTS
                            .iter()
                            .filter(|(_, document)| {
                                split_text_into_lowercase_words(document).contains(word)
                            })
                            .count()
                            + 1) as f64;
                    assert!(
                        value > 0.0,
                        "логарифм определён только для положительных чисел"
                    );
                    if value == f64::INFINITY {
                        return f64::INFINITY;
                    }
                    let mut scaled: f64 = value;
                    let mut power_of_two: i32 = 0i32;
                    while scaled >= 2.0 {
                        scaled /= 2.0;
                        power_of_two += 1;
                    }
                    while scaled < 1.0 {
                        scaled *= 2.0;
                        power_of_two -= 1;
                    }
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        value: f64,
                    ) -> f64 {
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        let ratio_squared: f64 = ratio * ratio;
                        let mut term: f64 = ratio;
                        let mut logarithm_series_sum: f64 = 0.0;
                        for term_index in 0..40 {
                            logarithm_series_sum += term / (2 * term_index + 1) as f64;
                            term *= ratio_squared;
                        }
                        2.0 * logarithm_series_sum
                    }

                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64
                        * approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        )
                })() + 1.0;
                relevance_score += *text_unit_counts.get(word).unwrap_or(&0) as f64
                    / document_text_units.len() as f64
                    * inverse_document_frequency_as_word_rarity_weight;
            }
            if relevance_score > 0.0 {
                ranked_results.push((document_identifier, relevance_score));
            }
        }
        ranked_results.sort_by(|result1, result2| result2.1.total_cmp(&result1.1));
        ranked_results
    })()
    .into_iter()
    .take(2)
    .collect::<Vec<_>>();
}

// Чему учит этот урок:
// Учимся соединять приведение слов к нижнему регистру, подсчёт частот, учёт редкости и сортировку
// документов.
// Получаем лучшие совпадения по словам; косинусное сравнение в текущем примере не вычисляется.
