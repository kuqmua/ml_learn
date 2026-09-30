//! Урок 192. Преобразование номеров частей текста в контекст и оценки следующей части.
//! Связь с принятой терминологией: Учебный прямой проход GPT с фиксированными весами.

/// Применяет фиксированную выходную проекцию к состояниям decoder.
/// Выходные логиты декодера: контекстные векторы превращаем в три оценки следующего токена фиксированными весами.
pub fn convert_text_identifiers_to_context_then_to_next_token_scores(
    text_unit_identifiers: &[usize],
) -> Vec<[f64; 3]> {
    // Третий logit — среднее двух координат (веса 0.5 и 0.5) фиксированной выходной проекции.
    l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(
        text_unit_identifiers,
    )
    .into_iter()
    .map(|hidden| [hidden[0], hidden[1], (hidden[0] + hidden[1]) * 0.5])
    .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем, что добавление будущего токена не меняет предыдущие оценки.
    fn future_tokens_do_not_change_earlier_scores() {
        assert_eq!(
            super::convert_text_identifiers_to_context_then_to_next_token_scores(&[0])[0],
            super::convert_text_identifiers_to_context_then_to_next_token_scores(&[0, 1])[0]
        );
    }
}
