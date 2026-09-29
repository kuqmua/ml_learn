// Урок 40.1. Вычисление линейного рекуррентного состояния по последовательности входов.
// Последовательность обрабатывается линейным сканированием с компактным состоянием.

fn main() {
    let input = [1.0, 0.0, 0.0, 0.0];
    let states =
        part_206_lesson_40_calculate_linear_recurrent_state_over_input_sequence::calculate_linear_recurrent_state_sequence_from_inputs(
            &input, 0.5, 1.0,
        );
    assert_eq!(states, [1.0, 0.5, 0.25, 0.125]);
    println!("затухание состояния: {states:?}");
    visualize_calculate_linear_recurrent_state_over_input_sequence(&states);
}

fn visualize_calculate_linear_recurrent_state_over_input_sequence(states: &[f64]) {
    let points: Vec<_> = states
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "state-space",
        "Затухание состояния",
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
