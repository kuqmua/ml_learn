// Урок 35.6. Вычисление выходных логитов учебной GPT по идентификаторам токенов.
// Собираем токены, позиции, причинное внимание и выходные логиты в один decoder-only проход.

fn main() {
    lesson_trace::enable();
    let prefix: [usize; 2] = [0, 1];
    lesson_trace::trace_step!(prefix);
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_scores: Vec<[f64; 3]> = part_186_lesson_35_calculate_tiny_gpt_output_logits_from_token_ids::calculate_decoder_output_logits_for_token_ids(&prefix);
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
        part_186_lesson_35_calculate_tiny_gpt_output_logits_from_token_ids::calculate_decoder_output_logits_for_token_ids(&[0])[0],
        raw_model_scores[0]
    );
    println!("префикс: {prefix:?}; logits: {last:?}; следующий ID: {next}");
    // Весов из GPT здесь нет: это минимальный прямой проход с фиксированными параметрами.
}
