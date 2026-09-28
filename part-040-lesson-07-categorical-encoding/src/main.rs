// Урок 07.5. Кодирование категорий и неизвестное значение.
// Словарь категорий учим на train; новую категорию на validation отправляем в отдельный ID.

use std::collections::BTreeMap;
fn build_vocabulary(training_data: &[&str]) -> BTreeMap<String, usize> {
    // Набор известных модели текстовых единиц называют vocabulary.
    let mut known_text_units = BTreeMap::new();
    for &category in training_data {
        if !known_text_units.contains_key(category) {
            let category_identifier = known_text_units.len() + 1;
            known_text_units.insert(category.to_owned(), category_identifier);
        }
    }
    known_text_units
}
fn encode(known_text_units: &BTreeMap<String, usize>, values: &[&str]) -> Vec<usize> {
    values
        .iter()
        .map(|value| known_text_units.get(*value).copied().unwrap_or(0))
        .collect()
}
fn main() {
    let known_text_units = build_vocabulary(&["red", "blue", "red"]);
    let validation = encode(&known_text_units, &["blue", "green"]);
    assert_eq!(validation[1], 0);
    println!("словарь train={known_text_units:?}; validation ID={validation:?}; ID 0=unknown");
}
