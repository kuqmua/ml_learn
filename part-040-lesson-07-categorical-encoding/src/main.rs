// Урок 07.5. Кодирование категорий и неизвестное значение.
// Словарь категорий учим на train; новую категорию на validation отправляем в отдельный ID.

use std::collections::BTreeMap;
fn build_vocabulary(training_data: &[&str]) -> BTreeMap<String, usize> {
    let mut vocabulary = BTreeMap::new();
    for &category in training_data {
        if !vocabulary.contains_key(category) {
            let category_identifier = vocabulary.len() + 1;
            vocabulary.insert(category.to_owned(), category_identifier);
        }
    }
    vocabulary
}
fn encode(vocabulary: &BTreeMap<String, usize>, values: &[&str]) -> Vec<usize> {
    values
        .iter()
        .map(|value| vocabulary.get(*value).copied().unwrap_or(0))
        .collect()
}
fn main() {
    let vocabulary = build_vocabulary(&["red", "blue", "red"]);
    let validation = encode(&vocabulary, &["blue", "green"]);
    assert_eq!(validation[1], 0);
    println!("словарь train={vocabulary:?}; validation ID={validation:?}; ID 0=unknown");
}
