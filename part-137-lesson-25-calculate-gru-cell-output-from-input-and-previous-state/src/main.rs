// Урок 25.4. Вычисление выхода ячейки GRU по входу и предыдущему состоянию.
// Update gate выбирает между предыдущим состоянием и новым кандидатом.

fn sigmoid_activation_of_raw_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
// Оценку модели до преобразования в вероятность называют logit.
fn calculate_gated_recurrent_unit_state_from_input_and_previous_state(
    input: f64,
    previous: f64,
    update_gate_raw_score: f64,
) -> f64 {
    let reset: f64 = sigmoid_activation_of_raw_score(input);
    lesson_trace::trace_step!(reset);
    let candidate: f64 = (input + reset * previous).tanh();
    lesson_trace::trace_step!(candidate);
    let update: f64 = sigmoid_activation_of_raw_score(update_gate_raw_score);
    lesson_trace::trace_step!(update);
    (1.0 - update) * previous + update * candidate
}
fn main() {
    lesson_trace::enable();
    let previous: f64 = 0.8;
    lesson_trace::trace_step!(previous);
    let keep: f64 =
        calculate_gated_recurrent_unit_state_from_input_and_previous_state(-0.2, previous, -5.0);
    lesson_trace::trace_step!(keep);
    let replace: f64 =
        calculate_gated_recurrent_unit_state_from_input_and_previous_state(-0.2, previous, 5.0);
    lesson_trace::trace_step!(replace);
    assert!((keep - previous).abs() < (replace - previous).abs());
    println!("keep={keep:.3}; replace={replace:.3}");
}
