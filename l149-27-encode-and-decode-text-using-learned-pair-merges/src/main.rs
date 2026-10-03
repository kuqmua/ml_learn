// Урок 27.3. Кодирование и восстановление текста с помощью выученных объединений пар.
// Зачем здесь эта тема: После обучения правил нужно одинаково кодировать новый текст и
//   восстанавливать исходный.
// Почему код устроен так: Применяем слияния в обученном порядке и проверяем обратимость
//   декодирования.
// Представь: Тот же набор обученных слияний должен превратить новую строку в токены и затем вернуть
//   исходную строку.
// При кодировании важен порядок изученных слияний; декодирование собирает исходные байты.

use l148_27_train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair::BytePairEncoding;

fn main() {
    let model: BytePairEncoding =
        BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
            &["кот спит", "кот ест", "пёс спит"],
            24,
        );
    let unseen: &str = "кот 🐈 спит";

    assert_eq!(model
        .restore_text_by_joining_token_bytes_and_decoding_them(&model
        .encode_text_as_token_identifiers_by_converting_bytes_and_applying_learned_merges(unseen))
        .expect("не удалось восстановить текст: неизвестный ID токена или неверная последовательность UTF-8"), unseen);
    assert!(
        model
            .restore_text_by_joining_token_bytes_and_decoding_them(&[usize::MAX])
            .is_err()
    );
}
