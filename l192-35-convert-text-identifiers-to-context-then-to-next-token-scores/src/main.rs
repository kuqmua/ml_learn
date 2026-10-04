// Урок 35.7. Преобразование номеров частей текста в контекст и оценки следующей части.
// Зачем здесь эта тема: Чтобы связать текстовый вход с обучением, нужно пройти от id через decoder
//   до логитов словаря.
// Почему код устроен так: Используем крошечный словарь и фиксированные веса, чтобы формы и
//   причинность проверялись вручную.
// Представь: Несколько id проходят через decoder, а выходные логиты дают оценку каждому слову
//   словаря.
// Собираем токены, позиции, причинное внимание и выходные логиты в один decoder-only проход.

use l192_35_convert_text_identifiers_to_context_then_to_next_token_scores::convert_text_identifiers_to_context_then_to_next_token_scores;

fn main() {
    let prefix: [usize; 2] = [0, 1];
    let raw_model_scores: [[f64; 3]; 2] =
        convert_text_identifiers_to_context_then_to_next_token_scores(&prefix)
            .try_into()
            .expect("ожидался один набор оценок на каждый идентификатор префикса");
    let last: &[f64; 3] = &raw_model_scores[1];
    let _: usize = last
        .iter()
        .enumerate()
        .max_by(|candidate1, candidate2| candidate1.1.total_cmp(candidate2.1))
        .unwrap()
        .0;
    assert_eq!(
        convert_text_identifiers_to_context_then_to_next_token_scores(&[0])[0],
        raw_model_scores[0]
    );

    let chosen = last
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .unwrap()
        .0;
    println!("Префикс={prefix:?}; оценки следующего токена={last:?}; выбран номер={chosen}");
    assert!(last.iter().all(|&score| score <= last[chosen]));
}

// Чему учит этот урок:
// Учимся превращать контекстные векторы в оценки возможного следующего токена.
// Выбираем наиболее высокую оценку и проверяем независимость раннего результата от будущего
// продолжения.
