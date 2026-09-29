// Урок 26.1. Причинная свёртка одномерного сигнала.
// Зачем здесь эта тема: Для прогноза следующего элемента нельзя читать будущие элементы
//   последовательности.
// Почему код устроен так: Свертка использует только текущий и прошлые индексы, что проверяется
//   изменением будущего входа.
// Представь: Изменение сигнала завтра не должно менять прогноз, сделанный сегодня.
// Выход в момент t зависит от текущего и прошлых элементов, но не от будущего.

fn main() {
    lesson_trace::enable();
    let signal: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
    lesson_trace::trace_step!(signal);
    let output: Vec<f64> =
        part_138_lesson_26_causal_convolution_over_one_dimensional_signal::causal_convolution_of_one_dimensional_signal(
            &signal, 1.0, 2.0, 1,
        )
        .unwrap();
    lesson_trace::trace_step!(output);
    assert_eq!(output, [1.0, 4.0, 7.0, 10.0]);
    println!("сигнал: {signal:?}; causal conv: {output:?}");
    lesson_trace::disable();
    visualize_causal_convolution_over_one_dimensional_signal(&signal, &output);
}
fn visualize_causal_convolution_over_one_dimensional_signal(input: &[f64], output: &[f64]) {
    let first_plot_points: Vec<(f64, f64)> = input
        .iter()
        .enumerate()
        .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
        .collect();
    let second_plot_points: Vec<(f64, f64)> = output
        .iter()
        .enumerate()
        .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
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
