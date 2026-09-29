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
    let reset = sigmoid_activation_of_raw_score(input);
    let candidate = (input + reset * previous).tanh();
    let update = sigmoid_activation_of_raw_score(update_gate_raw_score);
    (1.0 - update) * previous + update * candidate
}
fn main() {
    let previous = 0.8;
    let keep =
        calculate_gated_recurrent_unit_state_from_input_and_previous_state(-0.2, previous, -5.0);
    let replace =
        calculate_gated_recurrent_unit_state_from_input_and_previous_state(-0.2, previous, 5.0);
    assert!((keep - previous).abs() < (replace - previous).abs());
    println!("keep={keep:.3}; replace={replace:.3}");
}
