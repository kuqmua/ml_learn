// Урок 35.6. Учебный прямой проход GPT.
// Собираем токены, позиции, причинное внимание и выходные логиты в один decoder-only проход.

use part_186_lesson_35_tiny_generative_pretrained_transformer_forward::calculate_forward_pass_for_text_units;

fn main() {
    let prefix = [0, 1];
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_scores = calculate_forward_pass_for_text_units(&prefix);
    let last = raw_model_scores.last().unwrap();
    let next = last
        .iter()
        .enumerate()
        .max_by(|first_candidate, second_candidate| first_candidate.1.total_cmp(second_candidate.1))
        .unwrap()
        .0;
    // Изменение будущего токена не меняет предыдущие позиции.
    assert_eq!(
        calculate_forward_pass_for_text_units(&[0])[0],
        raw_model_scores[0]
    );
    println!("префикс: {prefix:?}; logits: {last:?}; следующий ID: {next}");
    // Весов из GPT здесь нет: это минимальный прямой проход с фиксированными параметрами.
}
