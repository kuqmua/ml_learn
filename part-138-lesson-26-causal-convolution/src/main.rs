// Урок 26.1. Причинная свёртка одномерного сигнала.
// Выход в момент t зависит от текущего и прошлых элементов, но не от будущего.

use part_138_lesson_26_causal_convolution::causal_conv;
fn main() {
    let signal = [1.0, 2.0, 3.0, 4.0];
    let output = causal_conv(&signal, 1.0, 2.0, 1).unwrap();
    assert_eq!(output, [1.0, 4.0, 7.0, 10.0]);
    println!("сигнал: {signal:?}; causal conv: {output:?}");
    visualize(&signal, &output);
}
fn visualize(input: &[f64], output: &[f64]) {
    let a: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(i, &y)| (i as f64, y))
        .collect();
    let b: Vec<_> = output
        .iter()
        .enumerate()
        .map(|(i, &y)| (i as f64, y))
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
                points: &a,
            },
            lesson_visualization::Series {
                name: "выход",
                points: &b,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", path.display());
}
