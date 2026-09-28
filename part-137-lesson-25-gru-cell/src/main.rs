// Урок 25.4. Ячейка GRU.
// Update gate выбирает между предыдущим состоянием и новым кандидатом.

fn sigmoid(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
fn gru(input: f64, previous: f64, update_logit: f64) -> f64 {
    let reset = sigmoid(input);
    let candidate = (input + reset * previous).tanh();
    let update = sigmoid(update_logit);
    (1.0 - update) * previous + update * candidate
}
fn main() {
    let previous = 0.8;
    let keep = gru(-0.2, previous, -5.0);
    let replace = gru(-0.2, previous, 5.0);
    assert!((keep - previous).abs() < (replace - previous).abs());
    println!("keep={keep:.3}; replace={replace:.3}");
}
