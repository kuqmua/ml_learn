// Урок 27.2. Выделение частей текста объединением самых частых соседних пар, начиная с байтов.
// Связь с принятой терминологией: Обучение слияний байтового BPE по обучающему корпусу.
// Зачем здесь эта тема: Работать по одному байту можно всегда, но частые сочетания выгодно
//   объединять.
// Почему код устроен так: Считаем частоты соседних пар и обучаем последовательность BPE-слияний
//   только на train.
// Представь: Если пара байтов часто повторяется, BPE может заменить её одним новым токеном.
// Частые соседние токены сливаются в новый токен; словарь учится только на train.

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Повторяющийся корпус даёт устойчивые кандидаты на слияние.");
    let corpus: [&str; 3] = ["мама мыла", "мама дома", "мама мыла"];
    trace_step!(corpus);
    let model: l148_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding = l148_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
        &corpus, 16,
    );
    trace_step!(model);
    assert!(!model.merges.is_empty());
    trace_note!("Показываем, как растёт словарь после каждого слияния.");
    println!(
        "слияний: {}; размер словаря: {}",
        model.merges.len(),
        model.pieces.len()
    );
    for (index, piece) in model.pieces.iter().enumerate().skip(256) {
        trace_step!(index);
        trace_step!(piece);
        println!("новый токен {index}: {:?}", String::from_utf8_lossy(piece));
    }
}
