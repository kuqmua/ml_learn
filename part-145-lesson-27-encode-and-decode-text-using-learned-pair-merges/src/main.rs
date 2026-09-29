// Урок 27.3. Кодирование и восстановление текста с помощью выученных объединений пар.
// Связь с принятой терминологией: Кодирование и декодирование текста обученными слияниями BPE.
// Зачем здесь эта тема: После обучения правил нужно одинаково кодировать новый текст и
//   восстанавливать исходный.
// Почему код устроен так: Применяем слияния в обученном порядке и проверяем обратимость
//   декодирования.
// Представь: Тот же набор обученных слияний должен превратить новую строку в токены и затем вернуть
//   исходную строку.
// При кодировании важен порядок изученных слияний; декодирование собирает исходные байты.

fn main() {
    lesson_trace::enable();
    // Словарь обучаем на одной части текста и применяем к новой строке.
    let model: part_144_lesson_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding = part_144_lesson_27_learn_text_units_by_merging_most_frequent_adjacent_byte_pairs::BytePairEncoding::learn_merges_by_repeatedly_joining_most_frequent_adjacent_pair(
        &["кот спит", "кот ест", "пёс спит"],
        24,
    );
    lesson_trace::trace_step!(model);
    let unseen: &str = "кот 🐈 спит";
    lesson_trace::trace_step!(unseen);
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifiers: Vec<usize> =
        model.convert_text_bytes_to_identifiers_and_apply_learned_merges(unseen);
    lesson_trace::trace_step!(text_unit_identifiers);
    let reconstructed: String = model
        .join_bytes_for_identifiers_and_decode_text(&text_unit_identifiers)
        .expect("каждый ID принадлежит словарю");
    lesson_trace::trace_step!(reconstructed);
    assert_eq!(reconstructed, unseen);
    assert!(
        model
            .join_bytes_for_identifiers_and_decode_text(&[usize::MAX])
            .is_err()
    );
    println!("текст: {unseen}; ID: {text_unit_identifiers:?}; восстановлено: {reconstructed}");
}
