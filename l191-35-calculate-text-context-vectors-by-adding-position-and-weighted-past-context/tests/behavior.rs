#[test]
fn single_token_adds_its_own_context_and_empty_text_has_no_states() {
    assert!(l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[]).is_empty());
    assert_eq!(l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[0]), vec![[2.0, 0.0]]);
    assert_eq!(l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[1]), vec![[0.0, 2.0]]);
    assert_eq!(l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[2]), vec![[1.0, 1.0]]);
}

#[test]
fn two_tokens_use_position_and_softmax_weighted_past() {
    let states = l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[0, 1]);
    assert_eq!(states.len(), 2);
    assert_eq!(states[0], [2.0, 0.0]);
    // Для второй позиции q=[0.1,1]: q·k0=0.1, q·k1=1.01; масштаб sqrt(2).
    let first_weight = 1.0 / (1.0 + ((1.01_f64 - 0.1) / 2.0_f64.sqrt()).exp());
    let expected = [0.2 + 0.9 * first_weight, 2.0 - first_weight];
    for coordinate in 0..2 {
        assert!((states[1][coordinate] - expected[coordinate]).abs() < 1e-12);
    }
}

#[test]
fn future_tokens_do_not_change_any_previous_state() {
    let text = [0, 1, 2, 0];
    let all_states = l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&text);
    for length in 0..=text.len() {
        assert_eq!(l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&text[..length]).as_slice(), &all_states[..length]);
    }
}
