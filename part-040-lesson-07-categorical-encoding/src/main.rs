// Урок 07.5. Кодирование категорий и неизвестное значение.
// Словарь категорий учим на train; новую категорию на validation отправляем в отдельный ID.

use std::collections::BTreeMap;
fn fit_vocab(train: &[&str]) -> BTreeMap<String, usize> {
    let mut vocab = BTreeMap::new();
    for &category in train {
        if !vocab.contains_key(category) {
            let category_id = vocab.len() + 1;
            vocab.insert(category.to_owned(), category_id);
        }
    }
    vocab
}
fn encode(vocab: &BTreeMap<String, usize>, values: &[&str]) -> Vec<usize> {
    values
        .iter()
        .map(|value| vocab.get(*value).copied().unwrap_or(0))
        .collect()
}
fn main() {
    let vocab = fit_vocab(&["red", "blue", "red"]);
    let validation = encode(&vocab, &["blue", "green"]);
    assert_eq!(validation[1], 0);
    println!("словарь train={vocab:?}; validation ID={validation:?}; ID 0=unknown");
}
