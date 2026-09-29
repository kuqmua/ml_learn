// Урок 40.1. Вычисление линейного рекуррентного состояния по последовательности входов.
// Зачем здесь эта тема: Последовательность можно хранить в компактном состоянии вместо внимания ко
//   всему прошлому.
// Почему код устроен так: Обновляем линейное состояние по одному входу и наблюдаем вклад старых
//   элементов.
// Представь: После каждого элемента последовательности храним одно число, в котором остаётся след
//   прошлого.
// Последовательность обрабатывается линейным сканированием с компактным состоянием.

fn main() {
    lesson_trace::enable();
    let input: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
    lesson_trace::trace_step!(input);
    let states: Vec<f64> =
        part_206_lesson_40_calculate_linear_recurrent_state_over_input_sequence::calculate_linear_recurrent_state_sequence_from_inputs(
            &input, 0.5, 1.0,
        );
    lesson_trace::trace_step!(states);
    assert_eq!(states, [1.0, 0.5, 0.25, 0.125]);
    println!("затухание состояния: {states:?}");
    lesson_trace::disable();
    visualize_calculate_linear_recurrent_state_over_input_sequence(&states);
}

fn visualize_calculate_linear_recurrent_state_over_input_sequence(states: &[f64]) {
    let points: Vec<(f64, f64)> = states
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
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
