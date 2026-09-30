// Урок 35.7. Преобразование номеров частей текста в контекст и оценки следующей части.
// Связь с принятой терминологией: Вычисление выходных логитов учебной GPT по идентификаторам токенов.
// Зачем здесь эта тема: Чтобы связать текстовый вход с обучением, нужно пройти от id через decoder
//   до логитов словаря.
// Почему код устроен так: Используем крошечный словарь и фиксированные веса, чтобы формы и
//   причинность проверялись вручную.
// Представь: Несколько id проходят через decoder, а выходные логиты дают оценку каждому слову
//   словаря.
// Собираем токены, позиции, причинное внимание и выходные логиты в один decoder-only проход.

use l192_35_convert_text_identifiers_to_context_and_next_piece_scores::convert_text_identifiers_to_context_then_to_next_token_scores;

fn main() {
    let prefix: [usize; 2] = [0, 1];
    let raw_model_scores: [[f64; 3]; 2] =
        convert_text_identifiers_to_context_then_to_next_token_scores(&prefix)
            .try_into()
            .expect("на каждый идентификатор префикса приходится один набор оценок");
    let last: &[f64; 3] = &raw_model_scores[1];
    let _next: usize = last
        .iter()
        .enumerate()
        .max_by(|first_candidate, second_candidate| first_candidate.1.total_cmp(second_candidate.1))
        .unwrap()
        .0;
    assert_eq!(
        convert_text_identifiers_to_context_then_to_next_token_scores(&[0])[0],
        raw_model_scores[0]
    );
}
