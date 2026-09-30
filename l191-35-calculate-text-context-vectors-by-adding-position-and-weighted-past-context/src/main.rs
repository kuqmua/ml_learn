// Контекстные векторы текста: сложение вектора токена, позиции и прошлого контекста.

use l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context;

use lesson_trace::{enable_tracing, trace_step};

fn main() {
    enable_tracing();
    let identifiers = [0, 1];
    trace_step!(identifiers);
    let states =
        calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&identifiers);
    trace_step!(states);
    println!("Контекстные векторы до выходной проекции: {states:?}");
    assert_eq!(states[0], [2.0, 0.0]);
    let extended =
        calculate_text_context_vectors_by_adding_position_and_weighted_past_context(&[0, 1, 2]);
    assert_eq!(states.as_slice(), &extended[..2]);
    println!("Добавление будущего токена не меняет предыдущие состояния.");
}
