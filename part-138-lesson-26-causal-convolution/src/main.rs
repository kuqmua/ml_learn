// Урок 26.1. Причинная свёртка одномерного сигнала.
// Выход в момент t зависит от текущего и прошлых элементов, но не от будущего.

use part_138_lesson_26_causal_convolution::causal_convolution;
fn main() {
    let signal = [1.0, 2.0, 3.0, 4.0];
    let output = causal_convolution(&signal, 1.0, 2.0, 1).unwrap();
    assert_eq!(output, [1.0, 4.0, 7.0, 10.0]);
    println!("сигнал: {signal:?}; causal conv: {output:?}");
    visualize(&signal, &output);
}
fn visualize(input: &[f64], output: &[f64]) {
    let first_plot_points: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
        .collect();
    let second_plot_points: Vec<_> = output
        .iter()
        .enumerate()
        .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "causal-conv",
        "Причинная свёртка",
        "t",
        "амплитуда",
        &[
            lesson_visualization::Series {
                name: "вход",
                points: &first_plot_points,
            },
            lesson_visualization::Series {
                name: "выход",
                points: &second_plot_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", path.display());
}
