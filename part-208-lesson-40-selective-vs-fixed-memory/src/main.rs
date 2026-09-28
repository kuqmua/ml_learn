// Урок 40.3. Селективная и постоянная память.
// Сравниваем фиксированное затухание с входозависимым забыванием.

use part_206_lesson_40_state_space_model::scan;
fn selective(values: &[f64], reset: &[bool]) -> Vec<f64> {
    let mut state = 0.0;
    values
        .iter()
        .zip(reset)
        .map(|(&x, &clear)| {
            state = if clear { x } else { 0.8 * state + x };
            state
        })
        .collect()
}
fn main() {
    let values = [1.0, 0.0, 2.0, 0.0];
    let reset = [false, false, true, false];
    let fixed = scan(&values, 0.8, 1.0);
    let dynamic = selective(&values, &reset);
    assert!(fixed[2] > dynamic[2]);
    println!("fixed={fixed:?}; selective={dynamic:?}");
    visualize(&fixed, &dynamic);
}
fn visualize(fixed: &[f64], dynamic: &[f64]) {
    let a: Vec<_> = fixed
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as f64, x))
        .collect();
    let b: Vec<_> = dynamic
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as f64, x))
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
                points: &a,
            },
            lesson_visualization::Series {
                name: "selective",
                points: &b,
            },
        ],
    )
    .expect("график");
    println!("график: {}", path.display());
}
