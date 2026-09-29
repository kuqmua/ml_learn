// Урок 25.3. Вычисление выхода ячейки LSTM по входу и предыдущему состоянию.
// Forget/input/output gates отдельно управляют памятью и наблюдаемым состоянием.

fn sigmoid_activation_of_raw_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
fn calculate_long_short_term_memory_cell_output(
    input: f64,
    previous_cell: f64,
    forget_bias: f64,
) -> (f64, f64) {
    let forget = sigmoid_activation_of_raw_score(forget_bias);
    let insert = sigmoid_activation_of_raw_score(input);
    let candidate = input.tanh();
    let cell = forget * previous_cell + insert * candidate;
    let hidden = sigmoid_activation_of_raw_score(input) * cell.tanh();
    (cell, hidden)
}
fn main() {
    let remembered = calculate_long_short_term_memory_cell_output(0.0, 1.0, 5.0).0;
    let forgotten = calculate_long_short_term_memory_cell_output(0.0, 1.0, -5.0).0;
    assert!(remembered > forgotten);
    println!("ячейка при открытом forget={remembered:.3}, при закрытом={forgotten:.3}");
}
