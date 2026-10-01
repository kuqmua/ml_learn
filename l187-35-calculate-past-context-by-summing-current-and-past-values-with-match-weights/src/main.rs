// Урок 35.3. Контекст без будущих данных: сложение текущих и прошлых значений с весами совпадений запроса и ключей.
// Связь с принятой терминологией: Причинное внимание по предыдущим токенам последовательности.
// Зачем здесь эта тема: Decoder предсказывает продолжение по уже известному префиксу.
// Почему код устроен так: Ограничиваем внимание текущей и прошлыми позициями и проверяем
//   независимость от будущего.
// Представь: При прогнозе слова после «добрый» decoder не должен использовать ещё неизвестное
//   продолжение.
// Для позиции i softmax вычисляется только по позициям 0..=i.

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum;
use l187_35_calculate_past_context_by_summing_current_and_past_values_with_match_weights::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

fn main() {
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let context: [[f64; 2]; 3] =
        calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &states, &states, &states,
        )
        .unwrap()
        .try_into()
        .expect("ожидался один контекст на каждую из трёх позиций");
    assert_eq!(context[0], states[0]);
    plot_weights_assigned_only_to_current_and_past_positions(&states);
    for (_index, _state) in context.iter().enumerate() {}
}

fn plot_weights_assigned_only_to_current_and_past_positions(states: &[[f64; 2]; 3]) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "causal-attention",
        "Веса причинного внимания",
        &std::array::from_fn::<[f64; 3], 3, _>(|item_index| {
            let query_vector = states[item_index];
            let weights: Vec<f64> = calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
                &(0..=item_index)
                    .map(|past_index| {
                        (query_vector[0] * states[past_index][0]
                            + query_vector[1] * states[past_index][1])
                            / 2.0_f64.sqrt()
                    })
                    .collect::<Vec<_>>(),
            );
            std::array::from_fn(|past_index| {
                if past_index <= item_index {
                    weights[past_index]
                } else {
                    0.0
                }
            })
        })
        .iter()
        .map(|row| row.to_vec())
        .collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
