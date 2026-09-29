// Урок 40.3. Сравнение зависящего от входа и постоянного забывания состояния.
// Сравниваем фиксированное затухание с входозависимым забыванием.

fn calculate_state_sequence_with_input_dependent_reset(values: &[f64], reset: &[bool]) -> Vec<f64> {
    let mut state = 0.0;
    values
        .iter()
        .zip(reset)
        .map(|(&input_value, &clear)| {
            state = if clear {
                input_value
            } else {
                0.8 * state + input_value
            };
            state
        })
        .collect()
}
fn main() {
    let values = [1.0, 0.0, 2.0, 0.0];
    let reset = [false, false, true, false];
    let fixed =
        part_206_lesson_40_calculate_linear_recurrent_state_over_input_sequence::calculate_linear_recurrent_state_sequence_from_inputs(
            &values, 0.8, 1.0,
        );
    let dynamic = calculate_state_sequence_with_input_dependent_reset(&values, &reset);
    assert!(fixed[2] > dynamic[2]);
    println!("fixed={fixed:?}; selective={dynamic:?}");
    visualize_compare_input_dependent_and_fixed_state_forgetting(&fixed, &dynamic);
}
fn visualize_compare_input_dependent_and_fixed_state_forgetting(fixed: &[f64], dynamic: &[f64]) {
    let first_plot_points: Vec<_> = fixed
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let second_plot_points: Vec<_> = dynamic
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "memory",
        "Постоянное и выборочное забывание",
        "шаг",
        "состояние",
        &[
            lesson_visualization::Series {
                name: "fixed",
                points: &first_plot_points,
            },
            lesson_visualization::Series {
                name: "selective",
                points: &second_plot_points,
            },
        ],
    )
    .expect("график");
    println!("график: {}", path.display());
}
