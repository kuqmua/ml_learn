// Урок 36.6. Повторное использование контекста: сохранение прошлых ключей и значений при генерации.
// Связь с принятой терминологией: Кэширование векторов ключей и значений прошлых токенов при генерации.
// Зачем здесь эта тема: При генерации новые токены повторно обращаются к старому префиксу.
// Почему код устроен так: Сохраняем K/V прошлых позиций и считаем для нового шага только новые
//   значения.
// Представь: При добавлении нового токена старые K/V берём из кэша, а не вычисляем снова для всего
//   префикса.
// Сохраняем K/V прошлых токенов и сверяем последний выход с полным причинным пересчётом.

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum;
use l187_35_calculate_past_context_by_summing_current_and_past_values_with_match_weights::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

use lesson_trace::{enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    trace_step!(states);
    let full: Vec<[f64; 2]> =
        calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &states, &states, &states,
        )
        .unwrap();
    trace_step!(full);
    let mut cached_keys: Vec<[f64; 2]> = Vec::new();
    trace_step!(cached_keys);
    let mut cached_values: Vec<[f64; 2]> = Vec::new();
    trace_step!(cached_values);
    let mut cached_outputs: Vec<[f64; 2]> = Vec::new();
    trace_step!(cached_outputs);
    for &new_state in &states {
        trace_step!(new_state);
        cached_keys.push(new_state);
        cached_values.push(new_state);
        trace_note!("Оценку модели до преобразования в вероятность называют logit.");
        let raw_model_scores: Vec<f64> = cached_keys
            .iter()
            .map(|key| (new_state[0] * key[0] + new_state[1] * key[1]) / 2.0_f64.sqrt())
            .collect();
        trace_step!(raw_model_scores);
        let weights: Vec<f64> =
            calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
                &raw_model_scores,
            );
        trace_step!(weights);
        let output: [f64; 2] = weights.iter().zip(&cached_values).fold(
            [0.0; 2],
            |mut output_state, (&weight_value, cached_value)| {
                output_state[0] += weight_value * cached_value[0];
                trace_step!(output_state);
                output_state[1] += weight_value * cached_value[1];
                trace_step!(output_state);
                output_state
            },
        );
        trace_step!(output);
        cached_outputs.push(output);
    }
    for (cached, recomputed) in cached_outputs.iter().zip(full) {
        trace_step!(cached);
        trace_step!(recomputed);
        assert!((cached[0] - recomputed[0]).abs() < 1e-12);
        assert!((cached[1] - recomputed[1]).abs() < 1e-12);
    }
    println!("выходы с KV-cache: {cached_outputs:?}");
}
