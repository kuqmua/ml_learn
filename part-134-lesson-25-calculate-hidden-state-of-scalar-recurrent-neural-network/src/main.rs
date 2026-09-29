// Урок 25.1. Вычисление скрытого состояния скалярной рекуррентной сети.
// Состояние переносит информацию от предыдущих элементов последовательности.

fn main() {
    let input: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
    let history: Vec<f64> = part_134_lesson_25_calculate_hidden_state_of_scalar_recurrent_neural_network::calculate_recurrent_hidden_states_from_input_sequence(
        &input, 0.8, 0.7,
    );
    assert_eq!(history.len(), input.len());
    println!("состояния: {history:?}");
    visualize_calculate_hidden_state_of_scalar_recurrent_neural_network(&history);
}
fn visualize_calculate_hidden_state_of_scalar_recurrent_neural_network(states: &[f64]) {
    let points: Vec<(f64, f64)> = states
        .iter()
        .enumerate()
        .map(|(item_index, &hidden_state)| (item_index as f64, hidden_state))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "rnn-state",
        "Память RNN",
        "шаг",
        "h_t",
        &[lesson_visualization::Series {
            name: "состояние",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
