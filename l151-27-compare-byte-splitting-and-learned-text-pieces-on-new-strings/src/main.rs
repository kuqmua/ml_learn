// Урок 27.5. Сравнение разбиения на байты и выученные части текста на новых строках.
// Зачем здесь эта тема: Меньше токенов на обучающем тексте не гарантирует пользу для новых строк.
// Почему код устроен так: Сравниваем байтовое кодирование и BPE на ранее не встречавшихся примерах.
// Представь: Знакомая строка может стать короче в BPE, но новую строку всё равно нужно уметь
//   закодировать байтами.
// Сравниваем длину byte-level и BPE кодирования на train и новых строках.

use l148_27_train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair::BytePairEncoding;

fn main() {
    let training_data: [&str; 3] = ["кот спит", "кот ест", "пёс спит"];
    let model: BytePairEncoding =
        BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
            &training_data,
            30,
        );
    let validation: [&str; 2] = ["кот играет", "🐈 спит"];
    let rows: [(&str, usize, usize); 5] = training_data
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
        .collect::<Vec<_>>()
        .try_into()
        .expect("ожидалось три обучающие и две проверочные строки");
    for &(text, bytes, text_units) in &rows {
        assert!(text_units <= bytes);
        assert_eq!(
            model
                .restore_text_by_joining_token_bytes_and_decoding_them(
                    &model.encode_text_as_token_identifiers_by_converting_bytes_and_applying_learned_merges(text)
                )
                .unwrap(),
            text
        );
    }

    // Выполняем вычисления из примера.
    let _ = &rows;
}

// График строится отдельно от проверки кодирования.
