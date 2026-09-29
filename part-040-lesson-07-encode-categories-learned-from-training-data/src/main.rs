// Урок 07.5. Кодирование категорий, изученных на обучающих данных.
// Словарь категорий учим на train; новую категорию на validation отправляем в отдельный ID.

fn build_category_id_vocabulary_from_training_data(
    training_data: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    // Набор известных модели текстовых единиц называют vocabulary.
    let mut known_text_units = std::collections::BTreeMap::new();
    for &category in training_data {
        if !known_text_units.contains_key(category) {
            let category_identifier = known_text_units.len() + 1;
            known_text_units.insert(category.to_owned(), category_identifier);
        }
    }
    known_text_units
}
fn encode_categories_with_known_vocabulary(
    known_text_units: &std::collections::BTreeMap<String, usize>,
    values: &[&str],
) -> Vec<usize> {
    values
        .iter()
        .map(|value| known_text_units.get(*value).copied().unwrap_or(0))
        .collect()
}
fn main() {
    let known_text_units = build_category_id_vocabulary_from_training_data(&["red", "blue", "red"]);
    let validation = encode_categories_with_known_vocabulary(&known_text_units, &["blue", "green"]);
    assert_eq!(validation[1], 0);
    println!("словарь train={known_text_units:?}; validation ID={validation:?}; ID 0=unknown");
}
