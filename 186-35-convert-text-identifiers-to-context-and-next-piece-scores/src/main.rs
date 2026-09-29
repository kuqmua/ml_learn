// Урок 35.6. Преобразование номеров частей текста в контекст и оценки следующей части.
// Связь с принятой терминологией: Вычисление выходных логитов учебной GPT по идентификаторам токенов.
// Зачем здесь эта тема: Чтобы связать текстовый вход с обучением, нужно пройти от id через decoder
//   до логитов словаря.
// Почему код устроен так: Используем крошечный словарь и фиксированные веса, чтобы формы и
//   причинность проверялись вручную.
// Представь: Несколько id проходят через decoder, а выходные логиты дают оценку каждому слову
//   словаря.
// Собираем токены, позиции, причинное внимание и выходные логиты в один decoder-only проход.

fn main() {
    lesson_trace::enable();
    let prefix: [usize; 2] = [0, 1];
    lesson_trace::trace_step!(prefix);
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_scores: Vec<[f64; 3]> = l186_35_convert_text_identifiers_to_context_and_next_piece_scores::convert_text_identifiers_to_context_then_to_next_token_scores(&prefix);
    lesson_trace::trace_step!(raw_model_scores);
    let last: &[f64; 3] = raw_model_scores.last().unwrap();
    lesson_trace::trace_step!(last);
    let next: usize = last
        .iter()
        .enumerate()
        .max_by(|first_candidate, second_candidate| first_candidate.1.total_cmp(second_candidate.1))
        .unwrap()
        .0;
    lesson_trace::trace_step!(next);
    // Изменение будущего токена не меняет предыдущие позиции.
    assert_eq!(
        l186_35_convert_text_identifiers_to_context_and_next_piece_scores::convert_text_identifiers_to_context_then_to_next_token_scores(&[0])[0],
        raw_model_scores[0]
    );
    println!("префикс: {prefix:?}; logits: {last:?}; следующий ID: {next}");
    // Весов из GPT здесь нет: это минимальный прямой проход с фиксированными параметрами.
}
