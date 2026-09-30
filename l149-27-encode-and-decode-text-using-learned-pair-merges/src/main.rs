// Урок 27.3. Кодирование и восстановление текста с помощью выученных объединений пар.
// Связь с принятой терминологией: Кодирование и декодирование текста обученными слияниями BPE.
// Зачем здесь эта тема: После обучения правил нужно одинаково кодировать новый текст и
//   восстанавливать исходный.
// Почему код устроен так: Применяем слияния в обученном порядке и проверяем обратимость
//   декодирования.
// Представь: Тот же набор обученных слияний должен превратить новую строку в токены и затем вернуть
//   исходную строку.
// При кодировании важен порядок изученных слияний; декодирование собирает исходные байты.

use l148_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding;

fn main() {
    let model: BytePairEncoding =
        BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
            &["кот спит", "кот ест", "пёс спит"],
            24,
        );
    let unseen: &str = "кот 🐈 спит";
    let text_unit_identifiers: Vec<usize> = model
        .encode_text_as_token_identifiers_by_converting_bytes_and_applying_learned_merges(unseen);
    let reconstructed: String = model
        .restore_text_by_joining_token_bytes_and_decoding_them(&text_unit_identifiers)
        .expect("каждый ID принадлежит словарю");
    assert_eq!(reconstructed, unseen);
    assert!(
        model
            .restore_text_by_joining_token_bytes_and_decoding_them(&[usize::MAX])
            .is_err()
    );
}
