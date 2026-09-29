// Урок 27.5. Сравнение разбиения на байты и выученные части текста на новых строках.
// Связь с принятой терминологией: Оценка байтового и BPE токенизаторов на новых строках.
// Зачем здесь эта тема: Меньше токенов на обучающем тексте не гарантирует пользу для новых строк.
// Почему код устроен так: Сравниваем байтовое кодирование и BPE на ранее не встречавшихся примерах.
// Представь: Знакомая строка может стать короче в BPE, но новую строку всё равно нужно уметь
//   закодировать байтами.
// Сравниваем длину byte-level и BPE кодирования на train и новых строках.

fn main() {
    lesson_trace::enable();
    // Новые строки не участвуют в выборе слияний.
    let training_data: [&str; 3] = ["кот спит", "кот ест", "пёс спит"];
    lesson_trace::trace_step!(training_data);
    let validation: [&str; 2] = ["кот играет", "🐈 спит"];
    lesson_trace::trace_step!(validation);
    let model: part_144_lesson_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding = part_144_lesson_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
        &training_data,
        30,
    );
    lesson_trace::trace_step!(model);
    let rows: Vec<(&str, usize, usize)> = training_data
        .iter()
        .chain(validation.iter())
        .map(|&text| {
            (
                text,
                text.len(),
                model
                    .encode_text_as_token_identifiers_by_converting_bytes_and_applying_learned_merges(text)
                    .len(),
            )
        })
        .collect();
    lesson_trace::trace_step!(rows);
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    for &(text, bytes, text_units) in &rows {
        lesson_trace::trace_step!(text);
        lesson_trace::trace_step!(bytes);
        lesson_trace::trace_step!(text_units);
        assert!(text_units <= bytes);
        assert_eq!(
            model
                .restore_text_by_joining_token_bytes_and_decoding_them(
                    &model.encode_text_as_token_identifiers_by_converting_bytes_and_applying_learned_merges(text)
                )
                .unwrap(),
            text
        );
        println!("{text:?}: байтов={bytes}, BPE-токенов={text_units}");
    }
    lesson_trace::disable();
    plot_number_of_text_units_after_learned_pair_merges(&rows);
}

// График строится отдельно от проверки кодирования.
fn plot_number_of_text_units_after_learned_pair_merges(rows: &[(&str, usize, usize)]) {
    let values: Vec<(&str, f64)> = rows
        .iter()
        .map(|(text, _, count)| (*text, *count as f64))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "token-count",
        "Длина BPE-кодирования",
        "число токенов",
        &values,
    )
    .expect("не удалось построить график");
    println!("график: {}", path.display());
}
