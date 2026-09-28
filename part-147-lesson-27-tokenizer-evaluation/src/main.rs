// Урок 27.5.191: Проверка токенизатора на новых строках.
// Сравниваем длину byte-level и BPE кодирования на train и новых строках.

use part_144_lesson_27_bpe_training::BytePairEncoding;

fn main() {
    // Новые строки не участвуют в выборе слияний.
    let training_data = ["кот спит", "кот ест", "пёс спит"];
    let validation = ["кот играет", "🐈 спит"];
    let model = BytePairEncoding::train_from_corpus(&training_data, 30);
    let rows: Vec<(&str, usize, usize)> = training_data
        .iter()
        .chain(validation.iter())
        .map(|&text| (text, text.len(), model.encode(text).len()))
        .collect();
    for &(text, bytes, tokens) in &rows {
        assert!(tokens <= bytes);
        assert_eq!(model.decode(&model.encode(text)).unwrap(), text);
        println!("{text:?}: байтов={bytes}, BPE-токенов={tokens}");
    }
    visualize(&rows);
}

// График строится отдельно от проверки кодирования.
fn visualize(rows: &[(&str, usize, usize)]) {
    let values: Vec<(&str, f64)> = rows
        .iter()
        .map(|(text, _, count)| (*text, *count as f64))
        .collect();
    let path = lesson_visualization::bars(
        env!("CARGO_MANIFEST_DIR"),
        "token-count",
        "Длина BPE-кодирования",
        "число токенов",
        &values,
    )
    .expect("не удалось построить график");
    println!("график: {}", path.display());
}
