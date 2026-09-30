// Урок 07.5. Числовое кодирование категорий: назначение номеров категориям из обучающих данных.
// Связь с принятой терминологией: Кодирование категорий, изученных на обучающих данных.
// Зачем здесь эта тема: Модели нужен числовой код категории, но словарь из test раскрыл бы будущие
//   значения.
// Почему код устроен так: Строим отображение только по train и отдельно обрабатываем неизвестную
//   категорию.
// Представь: Если в train не было города «Казань», его появление при прогнозе не должно
//   перестраивать словарь известных городов.
// Словарь категорий учим на train; новую категорию на validation отправляем в отдельный ID.

/// Словарь категорий: каждой новой категории обучения назначаем номер, начиная с 1.

fn build_category_dictionary_by_assigning_identifiers_to_unique_training_categories(
    training_data: &[&str],
) -> std::collections::BTreeMap<String, usize> {
    let mut known_text_units: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for &category in training_data {
        if !known_text_units.contains_key(category) {
            let category_identifier: usize = known_text_units.len() + 1;
            known_text_units.insert(category.to_owned(), category_identifier);
        }
    }
    known_text_units
}
/// Кодирование категорий: заменяем известные категории номерами, неизвестные — нулём.
fn encode_categories_by_replacing_with_known_identifiers_or_zero<const N: usize>(
    known_text_units: &std::collections::BTreeMap<String, usize>,
    values: &[&str; N],
) -> [usize; N] {
    std::array::from_fn(|index| known_text_units.get(values[index]).copied().unwrap_or(0))
}
fn main() {
    let known_text_units: std::collections::BTreeMap<String, usize> =
        build_category_dictionary_by_assigning_identifiers_to_unique_training_categories(&[
            "red", "blue", "red",
        ]);
    let validation: [usize; 2] = encode_categories_by_replacing_with_known_identifiers_or_zero(
        &known_text_units,
        &["blue", "green"],
    );
    assert_eq!(validation[1], 0);
}
