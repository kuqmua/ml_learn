// Урок 26.3. Управляемая активация с ветками tanh и sigmoid.
// Одна ветка tanh создаёт сигнал, другая sigmoid управляет его пропусканием.

fn sigmoid_activation_of_raw_score(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}
fn calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter: f64, gate: f64) -> f64 {
    filter.tanh() * sigmoid_activation_of_raw_score(gate)
}
fn main() {
    let filter = 1.5;
    let open = calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter, 5.0);
    let closed = calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter, -5.0);
    assert!(open > closed);
    assert!(closed >= 0.0);
    println!("закрытый gate={closed:.4}; открытый gate={open:.4}");
    visualize_gated_activation_with_tanh_and_sigmoid_branches(filter);
}

fn visualize_gated_activation_with_tanh_and_sigmoid_branches(filter: f64) {
    let points: Vec<_> = (-50..=50)
        .map(|plot_step_index| {
            let gate = plot_step_index as f64 / 10.0;
            (
                gate,
                calculate_wavenet_gated_activation_from_tanh_and_sigmoid(filter, gate),
            )
        })
        .collect();
    let path = lesson_visualization::line_chart(
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
