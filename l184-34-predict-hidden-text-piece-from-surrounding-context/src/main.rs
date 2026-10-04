// Урок 34.3. Прогноз скрытой части текста по окружающему контексту.
// Зачем здесь эта тема: Двунаправленный encoder можно обучать восстанавливать скрытый токен по
//   обоим соседним контекстам.
// Почему код устроен так: Скрываем одну позицию и проверяем прогноз без доступа к её исходному
//   значению.
// Представь: В «кошка [MASK] молоко» encoder использует слова с обеих сторон, чтобы угадать
//   пропуск.
// Цель содержит только скрытые позиции, а encoder видит левый и правый контекст.

use l182_34_calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores::calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores;

fn main() {
    let original: [usize; 3] = [0, 1, 2];
    let dense_numeric_representations: [[f64; 2]; 4] =
        [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [0.0, 0.0]];
    let visible: [[f64; 2]; 3] = [
        dense_numeric_representations[original[0]],
        dense_numeric_representations[3],
        dense_numeric_representations[original[2]],
    ];
    let context: [[f64; 2]; 3] =
        calc_visible_context_by_summing_states_weighted_by_exponentiated_coord_scores(
            &visible, &[true; 3],
        )
        .unwrap();
    let hidden_text_unit_identifier: usize = original[1];
    let raw_model_scores: [f64; 3] = std::array::from_fn(|index| {
        let candidate = dense_numeric_representations[index];
        context[1][0] * candidate[0] + context[1][1] * candidate[1]
    });
    let maximum_value: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let logarithm_of_sum_of_exponentials: f64 = maximum_value
        + raw_model_scores
            .iter()
            .map(|value| (value - maximum_value).exp())
            .sum::<f64>()
            .ln();
    let neg_log_hidden_token_probability: f64 =
        logarithm_of_sum_of_exponentials - raw_model_scores[hidden_text_unit_identifier];
    assert!(neg_log_hidden_token_probability.is_finite());
}

// Чему учит этот урок:
// Учимся скрывать токен, вычислять окружающий контекст и оценивать вероятность исходного токена.
// Получаем ошибку восстановления скрытого элемента — основу задачи обучения по пропускам в тексте.
