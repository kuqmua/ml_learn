//! Урок 186. Преобразование номеров частей текста в контекст и оценки следующей части.
//! Связь с принятой терминологией: Учебный прямой проход GPT с фиксированными весами.

/// Возвращает состояния после причинного внимания и residual.
// Единицу текста, которую модель обрабатывает как одно целое, называют token.
/// Скрытые состояния учебного декодера: берём векторы по номерам токенов, добавляем позицию и взвешенный контекст без будущих позиций.
pub fn calculate_text_context_vectors_by_adding_position_and_weighted_past_context(
    text_unit_identifiers: &[usize],
) -> Vec<[f64; 2]> {
    // Три двумерных embedding заданы вручную: так весь прямой проход можно просчитать на бумаге.
    let dense_numeric_representation: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    lesson_trace::trace_step!(dense_numeric_representation);
    // Позиционный вклад 0.1·position добавляем только к первой координате для наглядного примера.
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
    lesson_trace::trace_step!(states);
    if states.is_empty() {
        return Vec::new();
    }
    let context: Vec<[f64; 2]> =
        part_182_lesson_35_sum_current_and_past_values_with_query_key_match_weights::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(&states, &states, &states)
            .unwrap();
    lesson_trace::trace_step!(context);
    states
        .iter()
        .zip(&context)
        .map(|(&state, &attended)| [state[0] + attended[0], state[1] + attended[1]])
        .collect()
}

/// Применяет фиксированную выходную проекцию к состояниям decoder.
/// Выходные логиты декодера: контекстные векторы превращаем в три оценки следующего токена фиксированными весами.
pub fn convert_text_identifiers_to_context_then_to_next_token_scores(
    text_unit_identifiers: &[usize],
) -> Vec<[f64; 3]> {
    // Третий logit — среднее двух координат (веса 0.5 и 0.5) фиксированной выходной проекции.
    calculate_text_context_vectors_by_adding_position_and_weighted_past_context(
        text_unit_identifiers,
    )
    .into_iter()
    .map(|hidden| [hidden[0], hidden[1], (hidden[0] + hidden[1]) * 0.5])
    .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем, что добавление будущего токена не меняет предыдущие состояния и оценки.
    fn future_tokens_do_not_change_earlier_states_or_scores() {
        assert_eq!(
            super::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[
                0
            ])[0],
            super::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[
                0, 1
            ])[0]
        );
        assert_eq!(
            super::convert_text_identifiers_to_context_then_to_next_token_scores(&[0])[0],
            super::convert_text_identifiers_to_context_then_to_next_token_scores(&[0, 1])[0]
        );
    }
}
