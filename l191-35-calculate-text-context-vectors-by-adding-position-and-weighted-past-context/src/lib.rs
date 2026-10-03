//! Урок 191. Контекстные векторы текста: сложение вектора токена, позиции и прошлого контекста.

/// Возвращает состояния после причинного внимания и прибавление входа.
// Единицу текста, которую модель обрабатывает как одно целое, называют token.
/// Скрытые состояния учебного декодера: берём векторы по номерам токенов, добавляем позицию и взвешенный контекст без будущих позиций.
/// Возвращает по одному состоянию на каждый идентификатор текста; длина текста переменна.
use l187_35_calculate_past_context_by_summing_current_and_past_values_with_match_weights::calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

pub fn calc_text_context_vecs_by_adding_position_and_weighted_past_context(
    text_unit_identifiers: &[usize],
) -> Vec<[f64; 2]> {
    let dense_numeric_representation: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    let states: Vec<[f64; 2]> = text_unit_identifiers
        .iter()
        .enumerate()
        .map(|(position, &item_identifier)| {
            [
                dense_numeric_representation[item_identifier][0] + position as f64 * 0.1,
                dense_numeric_representation[item_identifier][1],
            ]
        })
        .collect();
    if states.is_empty() {
        return Vec::new();
    }

    states
        .iter()
        .zip(
            &calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
                &states, &states, &states,
            )
            .unwrap(),
        )
        .map(|(&state, &attended)| [state[0] + attended[0], state[1] + attended[1]])
        .collect()
}
