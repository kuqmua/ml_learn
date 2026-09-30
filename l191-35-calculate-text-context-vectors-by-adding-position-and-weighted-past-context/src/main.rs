// Контекстные векторы текста: сложение вектора токена, позиции и прошлого контекста.

use l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context;

fn main() {
    let identifiers = [0, 1];
    let states =
        calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&identifiers);

    assert_eq!(states[0], [2.0, 0.0]);
    let extended =
        calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[0, 1, 2]);
    assert_eq!(states.as_slice(), &extended[..2]);
}
