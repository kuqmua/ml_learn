// Урок 25.4. Ячейка GRU.
// Update gate выбирает между предыдущим состоянием и новым кандидатом.

fn sigmoid(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
// Оценку модели до преобразования в вероятность называют logit.
fn gated_recurrent_unit_output(input: f64, previous: f64, update_gate_raw_score: f64) -> f64 {
    let reset = sigmoid(input);
    let candidate = (input + reset * previous).tanh();
    let update = sigmoid(update_gate_raw_score);
    (1.0 - update) * previous + update * candidate
}
fn main() {
    let previous = 0.8;
    let keep = gated_recurrent_unit_output(-0.2, previous, -5.0);
    let replace = gated_recurrent_unit_output(-0.2, previous, 5.0);
    assert!((keep - previous).abs() < (replace - previous).abs());
    println!("keep={keep:.3}; replace={replace:.3}");
}
