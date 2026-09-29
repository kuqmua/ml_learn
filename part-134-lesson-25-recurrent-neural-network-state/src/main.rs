// Урок 25.1. Скрытое состояние RNN.
// Состояние переносит информацию от предыдущих элементов последовательности.

fn main() {
    let input = [1.0, 0.0, 0.0, 0.0];
    let history = part_134_lesson_25_recurrent_neural_network_state::calculate_recurrent_states(
        &input, 0.8, 0.7,
    );
    assert_eq!(history.len(), input.len());
    println!("состояния: {history:?}");
    visualize(&history);
}
fn visualize(states: &[f64]) {
    let points: Vec<_> = states
        .iter()
        .enumerate()
        .map(|(item_index, &hidden_state)| (item_index as f64, hidden_state))
        .collect();
    let path = lesson_visualization::line_chart(
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
