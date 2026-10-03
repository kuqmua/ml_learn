// Урок 30.2. Значимость слова для поиска: умножение частоты на меру редкости среди документов.
// Зачем здесь эта тема: Частое слово во всех документах хуже различает темы, чем редкое слово.
// Почему код устроен так: Умножаем частоту слова в документе на вес обратной частоты по корпусу.
// Представь: Слово, встречающееся во всех документах, помогает поиску меньше, чем редкое
//   характерное слово.
//
// Что изучаем: TF-IDF.
// Зачем это нужно: Оценка повышает вес слова, частого в документе, и понижает вес слова, встречающегося во
// многих документах.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let document_count: f64 = 10.0;
    let documents_with_term: f64 = 2.0;
    let document_count_divided_by_documents_containing_word: f64 =
        (document_count + 1.0) / (documents_with_term + 1.0);
    let normalized: f64 = (document_count_divided_by_documents_containing_word - 1.0)
        / (document_count_divided_by_documents_containing_word + 1.0);
    let mut term: f64 = normalized;
    let mut logarithm: f64 = 0.0;
    for odd_divisor in (1..=99).step_by(2) {
        logarithm += term / odd_divisor as f64;
        term *= normalized * normalized;
    }
    let inverse_document_frequency_as_word_rarity_weight: f64 = 2.0 * logarithm;
    let word_count_in_document: f64 = 3.0;
    let _: f64 = word_count_in_document * inverse_document_frequency_as_word_rarity_weight;

    plot_word_frequency_weighted_by_rarity_across_documents(
        inverse_document_frequency_as_word_rarity_weight,
    );
}

// Строим график по результатам урока.
fn plot_word_frequency_weighted_by_rarity_across_documents(
    inverse_document_frequency_as_word_rarity_weight: f64,
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "TF-IDF",
        "частота токена",
        "оценка TF-IDF",
        &[lesson_visualization::Series {
            name: "idf из примера",

            points: &(0..=10)
                .map(|plot_step_index| {
                    let word_count_in_document: f64 = plot_step_index as f64 / 10.0;
                    (
                        word_count_in_document,
                        word_count_in_document * inverse_document_frequency_as_word_rarity_weight,
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
