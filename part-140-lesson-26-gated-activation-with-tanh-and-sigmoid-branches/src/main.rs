// Урок 26.3. Управляемая активация с ветками tanh и sigmoid.
// Почему этот урок сейчас: Сигналу может требоваться пропускать или подавлять найденный фильтром признак.
// Почему пример устроен так: Умножаем ветку tanh на сигмоидальные ворота и смотрим влияние gate.
// Одна ветка tanh создаёт сигнал, другая sigmoid управляет его пропусканием.

fn sigmoid_activation_of_raw_score(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}
fn calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter: f64, gate: f64) -> f64 {
    filter.tanh() * sigmoid_activation_of_raw_score(gate)
}
fn main() {
    lesson_trace::enable();
    let filter: f64 = 1.5;
    lesson_trace::trace_step!(filter);
    let open: f64 = calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter, 5.0);
    lesson_trace::trace_step!(open);
    let closed: f64 = calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter, -5.0);
    lesson_trace::trace_step!(closed);
    assert!(open > closed);
    assert!(closed >= 0.0);
    println!("закрытый gate={closed:.4}; открытый gate={open:.4}");
    lesson_trace::disable();
    visualize_gated_activation_with_tanh_and_sigmoid_branches(filter);
}

fn visualize_gated_activation_with_tanh_and_sigmoid_branches(filter: f64) {
    let points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            let gate: f64 = plot_step_index as f64 / 10.0;
            (
                gate,
                calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter, gate),
            )
        })
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "gate",
        "Управляемая активация",
        "gate",
        "выход",
        &[lesson_visualization::Series {
            name: "tanh(filter)*sigmoid_activation_of_raw_score(gate)",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
