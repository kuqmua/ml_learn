// Урок 27.3.189: Кодирование и декодирование BPE.
// При кодировании важен порядок изученных слияний; декодирование собирает исходные байты.

fn main() {
    // Словарь обучаем на одной части текста и применяем к новой строке.
    let model = part_144_lesson_27_byte_pair_encoding_training::BytePairEncoding::train_from_corpus(
        &["кот спит", "кот ест", "пёс спит"],
        24,
    );
    let unseen = "кот 🐈 спит";
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifiers = model.encode_text_as_byte_pair_tokens(unseen);
    let reconstructed = model
        .decode_byte_pair_tokens_to_text(&text_unit_identifiers)
        .expect("каждый ID принадлежит словарю");
    assert_eq!(reconstructed, unseen);
    assert!(
        model
            .decode_byte_pair_tokens_to_text(&[usize::MAX])
            .is_err()
    );
    println!("текст: {unseen}; ID: {text_unit_identifiers:?}; восстановлено: {reconstructed}");
}
