// Урок 27.2. Обучение слияний байтового BPE по обучающему корпусу.
// Частые соседние токены сливаются в новый токен; словарь учится только на train.

fn main() {
    // Повторяющийся корпус даёт устойчивые кандидаты на слияние.
    let corpus: [&str; 3] = ["мама мыла", "мама дома", "мама мыла"];
    let model: part_144_lesson_27_train_byte_level_bpe_merges_from_training_corpus::BytePairEncoding = part_144_lesson_27_train_byte_level_bpe_merges_from_training_corpus::BytePairEncoding::train_byte_pair_encoding_merges_from_corpus(
        &corpus, 16,
    );
    assert!(!model.merges.is_empty());
    // Показываем, как растёт словарь после каждого слияния.
    println!(
        "слияний: {}; размер словаря: {}",
        model.merges.len(),
        model.pieces.len()
    );
    for (index, piece) in model.pieces.iter().enumerate().skip(256) {
        println!("новый токен {index}: {:?}", String::from_utf8_lossy(piece));
    }
}
