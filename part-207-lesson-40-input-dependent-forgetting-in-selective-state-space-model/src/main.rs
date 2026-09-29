// Урок 40.2. Забывание состояния с коэффициентом, зависящим от входа.
// Вход управляет коэффициентом забывания; это учебная идея selective SSM, не реализация Mamba.

fn calculate_state_sequence_with_input_dependent_reset(input: &[(f64, bool)]) -> Vec<f64> {
    let mut state = 0.0;
    input
        .iter()
        .map(|&(value, reset)| {
            // Долю (fraction) предыдущего состояния, сохраняемую на следующем шаге, называют retention.
            let previous_state_share_kept = if reset { 0.0 } else { 0.9 };
            state = previous_state_share_kept * state + value;
            state
        })
        .collect()
}
fn main() {
    let sequence = [(1.0, false), (0.0, false), (2.0, true), (0.0, false)];
    let states = calculate_state_sequence_with_input_dependent_reset(&sequence);
    assert_eq!(states[0], 1.0);
    assert_eq!(states[2], 2.0); // reset удаляет прошлый контекст.
    println!("селективное состояние: {states:?}");
    visualize_input_dependent_forgetting_in_selective_state_space_model(&states);
}

fn visualize_input_dependent_forgetting_in_selective_state_space_model(states: &[f64]) {
    let points: Vec<_> = states
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "selective-state",
        "Сброс состояния на третьем шаге",
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
