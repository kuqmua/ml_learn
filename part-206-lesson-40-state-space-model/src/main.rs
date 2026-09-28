// Урок 40.1. Рекуррентная модель состояния.
// Последовательность обрабатывается линейным сканированием с компактным состоянием.

use part_206_lesson_40_state_space_model::scan;
fn main() {
    let input = [1.0, 0.0, 0.0, 0.0];
    let states = scan(&input, 0.5, 1.0);
    assert_eq!(states, [1.0, 0.5, 0.25, 0.125]);
    println!("затухание состояния: {states:?}");
    visualize(&states);
}

fn visualize(states: &[f64]) {
    let points: Vec<_> = states
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as f64, x))
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
