// Урок 36.6. Повторное использование контекста: сохранение прошлых ключей и значений при генерации.
// Зачем здесь эта тема: При генерации новые токены повторно обращаются к старому префиксу.
// Почему код устроен так: Сохраняем K/V прошлых позиций и считаем для нового шага только новые
//   значения.
// Представь: При добавлении нового токена старые K/V берём из кэша, а не вычисляем снова для всего
//   префикса.
// Сохраняем K/V прошлых токенов и сверяем последний выход с полным причинным пересчётом.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

use l186_35_calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum;
use l187_35_calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches::calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

fn main() {
    let mut cached_keys: Vec<[f64; 2]> = Vec::new();
    let mut cached_values: Vec<[f64; 2]> = Vec::new();
    let mut cached_outputs: Vec<[f64; 2]> = Vec::new();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    for &new_state in &states {
        cached_keys.push(new_state);
        cached_values.push(new_state);

        cached_outputs.push(
            calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
                &cached_keys
                    .iter()
                    .map(|key| (new_state[0] * key[0] + new_state[1] * key[1]) / 2.0_f64.sqrt())
                    .collect::<Vec<_>>(),
            )
            .iter()
            .zip(&cached_values)
            .fold(
                [0.0; 2],
                |mut output_state, (&weight_value, cached_value)| {
                    output_state[0] += weight_value * cached_value[0];
                    output_state[1] += weight_value * cached_value[1];
                    output_state
                },
            ),
        );
    }
    for (cached, recomputed) in cached_outputs.iter().zip(
        calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &states, &states, &states,
        )
        .unwrap(),
    ) {
        assert!(check_f64_eq_1e_minus_12(cached[0], recomputed[0]));
        assert!(check_f64_eq_1e_minus_12(cached[1], recomputed[1]));
    }
}
