// Урок 34.3. Прогноз скрытой части текста по окружающему контексту.
// Связь с принятой терминологией: Предсказание скрытого токена по двустороннему контексту encoder.
// Зачем здесь эта тема: Двунаправленный encoder можно обучать восстанавливать скрытый токен по
//   обоим соседним контекстам.
// Почему код устроен так: Скрываем одну позицию и проверяем прогноз без доступа к её исходному
//   значению.
// Представь: В «кошка [MASK] молоко» encoder использует слова с обеих сторон, чтобы угадать
//   пропуск.
// Цель содержит только скрытые позиции, а encoder видит левый и правый контекст.

use l182_34_build_text_context_from_both_earlier_and_later_positions::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products;

use lesson_trace::{enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Три токена A, B, C; средний заменяем отдельным MASK embedding.");
    let original: [usize; 3] = [0, 1, 2];
    trace_step!(original);
    trace_note!("Плотное числовое представление объекта называют embedding.");
    let dense_numeric_representations: [[f64; 2]; 4] =
        [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [0.0, 0.0]];
    trace_step!(dense_numeric_representations);
    let visible: [[f64; 2]; 3] = [
        dense_numeric_representations[original[0]],
        dense_numeric_representations[3],
        dense_numeric_representations[original[2]],
    ];
    trace_step!(visible);
    let context: Vec<[f64; 2]> =
        calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
            &visible, &[true; 3],
        )
        .unwrap();
    trace_step!(context);
    trace_note!("Скрытие позиции для её предсказания называют masked language modeling.");
    let hidden_text_unit_identifier: usize = original[1];
    trace_step!(hidden_text_unit_identifier);
    trace_note!("Оценку модели до преобразования в вероятность называют logit.");
    let raw_model_scores: Vec<f64> = dense_numeric_representations[..3]
        .iter()
        .map(|candidate| context[1][0] * candidate[0] + context[1][1] * candidate[1])
        .collect();
    trace_step!(raw_model_scores);
    let maximum_value: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    trace_step!(maximum_value);
    let logarithm_of_sum_of_exponentials: f64 = maximum_value
        + raw_model_scores
            .iter()
            .map(|value| (value - maximum_value).exp())
            .sum::<f64>()
            .ln();
    trace_step!(logarithm_of_sum_of_exponentials);
    let loss: f64 =
        logarithm_of_sum_of_exponentials - raw_model_scores[hidden_text_unit_identifier];
    trace_step!(loss);
    assert!(loss.is_finite());
    println!("цель скрытой позиции={hidden_text_unit_identifier}; MLM loss={loss:.4}");
    trace_note!("Фиксированные embeddings иллюстрируют loss, а не обученный BERT.");
}
