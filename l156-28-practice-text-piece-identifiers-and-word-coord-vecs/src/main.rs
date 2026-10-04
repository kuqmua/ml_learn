// Урок 156. Соединять словарь, обработку неизвестных слов и получение векторов по идентификаторам.
// Векторы здесь заданы простой формулой, а не обучены: пример показывает устройство преобразования
// текста в числа.

fn main() {
    let known_text_units: std::collections::BTreeMap<String, usize> =
        (|| -> std::collections::BTreeMap<String, usize> {
            let corpus: [&str; 2] = ["кот спит", "пёс спит"];
            let corpus: &[&str] = &corpus;
            let mut known_text_units: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            known_text_units.insert("<unk>".into(), 0);
            for word in corpus
                .iter()
                .flat_map(|sentence| sentence.split_whitespace())
            {
                if !known_text_units.contains_key(word) {
                    known_text_units.insert(word.into(), known_text_units.len());
                }
            }
            known_text_units
        })();

    let mut dense_numeric_representations: Vec<[f64; 2]> = vec![[0.0, 0.0]; known_text_units.len()];
    for (text_unit_index, row) in dense_numeric_representations.iter_mut().enumerate() {
        *row = [text_unit_index as f64 * 0.1, text_unit_index as f64 * 0.2];
    }
    let text_unit_indices: Vec<usize> = (|| -> Vec<usize> {
        let known_text_units: &std::collections::BTreeMap<String, usize> = &known_text_units;
        let text: &str = "кот неизвестно";
        text.split_whitespace()
            .map(|word| *known_text_units.get(word).unwrap_or(&0))
            .collect()
    })();
    let _ = &(text_unit_indices
        .iter()
        .map(|&token_index| dense_numeric_representations[token_index])
        .collect::<Vec<_>>());

    // Выполняем вычисления из примера.
    let _ = (&known_text_units, &text_unit_indices);

    let vectors: Vec<_> = text_unit_indices
        .iter()
        .map(|&id| dense_numeric_representations[id])
        .collect();
    println!("Словарь={known_text_units:?}; номера={text_unit_indices:?}; векторы={vectors:?}");
    assert_eq!(text_unit_indices[1], 0);
    assert_eq!(vectors[1], dense_numeric_representations[0]);
}

// Чему учит этот урок:
// Учимся соединять словарь, обработку неизвестных слов и получение векторов по идентификаторам.
// Векторы здесь заданы простой формулой, а не обучены: пример показывает устройство преобразования
// текста в числа.
