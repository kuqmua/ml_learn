// Урок 148. Строить набор частей текста повторным объединением самых частых соседних пар.
// Получаем правила разбиения, позволяющие представлять частые фрагменты меньшим числом
// идентификаторов.

use l148_27_train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair::BytePairEncoding;

fn main() {
    let corpus: [&str; 3] = ["мама мыла", "мама дома", "мама мыла"];
    let model: BytePairEncoding =
        BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
            &corpus, 16,
        );
    assert!(!model.merges.is_empty());
    println!(
        "Обученный словарь частей текста: {:?}",
        (&(model.merges.len()), &(model.pieces.len()))
    );
    for (_index, piece) in model.pieces.iter().enumerate().skip(256) {
        println!(
            "Обученный словарь частей текста: {:?}",
            &(String::from_utf8_lossy(piece))
        );
    }
}

// Чему учит этот урок:
// Учимся строить набор частей текста повторным объединением самых частых соседних пар.
// Получаем правила разбиения, позволяющие представлять частые фрагменты меньшим числом
// идентификаторов.
