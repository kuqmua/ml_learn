// Урок 26.1. Скрытое состояние RNN.
// Состояние переносит информацию от предыдущих элементов последовательности.

use part_137_lesson_26_rnn_state::states;
fn main() {
    let input = [1.0, 0.0, 0.0, 0.0];
    let history = states(&input, 0.8, 0.7);
    assert_eq!(history.len(), input.len());
    println!("состояния: {history:?}");
    visualize(&history);
}
fn visualize(states: &[f64]) {
    let points: Vec<_> = states
        .iter()
        .enumerate()
        .map(|(i, &h)| (i as f64, h))
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
