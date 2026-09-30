// Урок 27.2. Выделение частей текста объединением самых частых соседних пар, начиная с байтов.
// Связь с принятой терминологией: Обучение слияний байтового BPE по обучающему корпусу.
// Зачем здесь эта тема: Работать по одному байту можно всегда, но частые сочетания выгодно
//   объединять.
// Почему код устроен так: Считаем частоты соседних пар и обучаем последовательность BPE-слияний
//   только на train.
// Представь: Если пара байтов часто повторяется, BPE может заменить её одним новым токеном.
// Частые соседние токены сливаются в новый токен; словарь учится только на train.

use l148_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding;

fn main() {
    let corpus: [&str; 3] = ["мама мыла", "мама дома", "мама мыла"];
    let model: BytePairEncoding =
        BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
            &corpus, 16,
        );
    assert!(!model.merges.is_empty());
    let _ = (&(model.merges.len()), &(model.pieces.len()));
    for (_index, piece) in model.pieces.iter().enumerate().skip(256) {
        let _ = &(String::from_utf8_lossy(piece));
    }
}
