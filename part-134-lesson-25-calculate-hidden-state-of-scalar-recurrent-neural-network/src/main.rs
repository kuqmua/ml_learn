// Урок 25.1. Вычисление скрытого состояния скалярной рекуррентной сети.
// Почему этот урок сейчас: Последовательность требует памяти о предыдущих входах; обычный слой обрабатывает элементы независимо.
// Почему пример устроен так: Обновляем одно скрытое состояние по очереди, чтобы прошлые элементы влияли на текущий выход.
// Состояние переносит информацию от предыдущих элементов последовательности.

fn main() {
    lesson_trace::enable();
    let input: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
    lesson_trace::trace_step!(input);
    let history: Vec<f64> = part_134_lesson_25_calculate_hidden_state_of_scalar_recurrent_neural_network::calculate_recurrent_hidden_states_from_input_sequence(
        &input, 0.8, 0.7,
    );
    lesson_trace::trace_step!(history);
    assert_eq!(history.len(), input.len());
    println!("состояния: {history:?}");
    lesson_trace::disable();
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
