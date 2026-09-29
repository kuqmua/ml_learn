// Урок 27.3. Кодирование и декодирование текста обученными слияниями BPE.
// Почему этот урок сейчас: После обучения правил нужно одинаково кодировать новый текст и восстанавливать исходный.
// Почему пример устроен так: Применяем слияния в обученном порядке и проверяем обратимость декодирования.
// При кодировании важен порядок изученных слияний; декодирование собирает исходные байты.

fn main() {
    lesson_trace::enable();
    // Словарь обучаем на одной части текста и применяем к новой строке.
    let model: part_144_lesson_27_train_byte_level_bpe_merges_from_training_corpus::BytePairEncoding = part_144_lesson_27_train_byte_level_bpe_merges_from_training_corpus::BytePairEncoding::train_byte_pair_encoding_merges_from_corpus(
        &["кот спит", "кот ест", "пёс спит"],
        24,
    );
    lesson_trace::trace_step!(model);
    let unseen: &str = "кот 🐈 спит";
    lesson_trace::trace_step!(unseen);
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifiers: Vec<usize> = model.encode_text_as_byte_pair_tokens(unseen);
    lesson_trace::trace_step!(text_unit_identifiers);
    let reconstructed: String = model
        .decode_byte_pair_tokens_to_text(&text_unit_identifiers)
        .expect("каждый ID принадлежит словарю");
    lesson_trace::trace_step!(reconstructed);
    assert_eq!(reconstructed, unseen);
    assert!(
        model
            .decode_byte_pair_tokens_to_text(&[usize::MAX])
            .is_err()
    );
    println!("текст: {unseen}; ID: {text_unit_identifiers:?}; восстановлено: {reconstructed}");
}
