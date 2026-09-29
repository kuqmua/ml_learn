// Урок 27.2. Обучение слияний байтового BPE по обучающему корпусу.
// Почему этот урок сейчас: Работать по одному байту можно всегда, но частые сочетания выгодно объединять.
// Почему пример устроен так: Считаем частоты соседних пар и обучаем последовательность BPE-слияний только на train.
// Частые соседние токены сливаются в новый токен; словарь учится только на train.

fn main() {
    lesson_trace::enable();
    // Повторяющийся корпус даёт устойчивые кандидаты на слияние.
    let corpus: [&str; 3] = ["мама мыла", "мама дома", "мама мыла"];
    lesson_trace::trace_step!(corpus);
    let model: part_144_lesson_27_train_byte_level_bpe_merges_from_training_corpus::BytePairEncoding = part_144_lesson_27_train_byte_level_bpe_merges_from_training_corpus::BytePairEncoding::train_byte_pair_encoding_merges_from_corpus(
        &corpus, 16,
    );
    lesson_trace::trace_step!(model);
    assert!(!model.merges.is_empty());
    // Показываем, как растёт словарь после каждого слияния.
    println!(
        "слияний: {}; размер словаря: {}",
        model.merges.len(),
        model.pieces.len()
    );
    for (index, piece) in model.pieces.iter().enumerate().skip(256) {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(piece);
        println!("новый токен {index}: {:?}", String::from_utf8_lossy(piece));
    }
}
