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

fn main() {
    let mut cached_keys: Vec<[f64; 2]> = Vec::new();
    let mut cached_values: Vec<[f64; 2]> = Vec::new();
    let mut cached_outputs: Vec<[f64; 2]> = Vec::new();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    for &new_state in &states {
        cached_keys.push(new_state);
        cached_values.push(new_state);

        cached_outputs.push(calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
                &cached_keys
            .iter()
            .map(|key| (new_state[0] * key[0] + new_state[1] * key[1]) / 2.0_f64.sqrt())
            .collect::<Vec<_>>(),
            ).iter().zip(&cached_values).fold(
            [0.0; 2],
            |mut output_state, (&weight_value, cached_value)| {
                output_state[0] += weight_value * cached_value[0];
                output_state[1] += weight_value * cached_value[1];
                output_state
            },
        ));
    }
    for (cached, recomputed) in cached_outputs.iter().zip(
        calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &states, &states, &states,
        )
        .unwrap(),
    ) {
        assert!((cached[0] - recomputed[0]).abs() < 1e-12);
        assert!((cached[1] - recomputed[1]).abs() < 1e-12);
    }
}
