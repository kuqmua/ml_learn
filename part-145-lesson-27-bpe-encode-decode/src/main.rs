// Урок 27.3.189: Кодирование и декодирование BPE.
// При кодировании важен порядок изученных слияний; декодирование собирает исходные байты.

use part_144_lesson_27_bpe_training::BytePairEncoding;

fn main() {
    // Словарь обучаем на одной части текста и применяем к новой строке.
    let model = BytePairEncoding::train_from_corpus(&["кот спит", "кот ест", "пёс спит"], 24);
    let unseen = "кот 🐈 спит";
    let token_identifiers = model.encode(unseen);
    let reconstructed = model
        .decode(&token_identifiers)
        .expect("каждый ID принадлежит словарю");
    assert_eq!(reconstructed, unseen);
    assert!(model.decode(&[usize::MAX]).is_err());
    println!("текст: {unseen}; ID: {token_identifiers:?}; восстановлено: {reconstructed}");
}
