// Урок 27.2.188: Обучение byte-level BPE.
// Частые соседние токены сливаются в новый токен; словарь учится только на train.

fn main() {
    // Повторяющийся корпус даёт устойчивые кандидаты на слияние.
    let corpus = ["мама мыла", "мама дома", "мама мыла"];
    let model = part_144_lesson_27_bpe_training::Bpe::train(&corpus, 16);
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
