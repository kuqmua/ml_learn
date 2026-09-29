// Урок 27.5. Оценка байтового и BPE токенизаторов на новых строках.
// Сравниваем длину byte-level и BPE кодирования на train и новых строках.

fn main() {
    // Новые строки не участвуют в выборе слияний.
    let training_data = ["кот спит", "кот ест", "пёс спит"];
    let validation = ["кот играет", "🐈 спит"];
    let model = part_144_lesson_27_train_byte_level_bpe_merges_from_training_corpus::BytePairEncoding::train_byte_pair_encoding_merges_from_corpus(
        &training_data,
        30,
    );
    let rows: Vec<(&str, usize, usize)> = training_data
        .iter()
        .chain(validation.iter())
        .map(|&text| {
            (
                text,
                text.len(),
                model.encode_text_as_byte_pair_tokens(text).len(),
            )
        })
        .collect();
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    for &(text, bytes, text_units) in &rows {
        assert!(text_units <= bytes);
        assert_eq!(
            model
                .decode_byte_pair_tokens_to_text(&model.encode_text_as_byte_pair_tokens(text))
                .unwrap(),
            text
        );
        println!("{text:?}: байтов={bytes}, BPE-токенов={text_units}");
    }
    visualize_evaluate_byte_level_and_bpe_tokenizers_on_new_strings(&rows);
}

// График строится отдельно от проверки кодирования.
fn visualize_evaluate_byte_level_and_bpe_tokenizers_on_new_strings(rows: &[(&str, usize, usize)]) {
    let values: Vec<(&str, f64)> = rows
        .iter()
        .map(|(text, _, count)| (*text, *count as f64))
        .collect();
    let path = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "token-count",
        "Длина BPE-кодирования",
        "число токенов",
        &values,
    )
    .expect("не удалось построить график");
    println!("график: {}", path.display());
}
