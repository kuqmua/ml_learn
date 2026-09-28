// Урок 39.3. Управляемая активация WaveNet.
// Одна ветка tanh создаёт сигнал, другая sigmoid управляет его пропусканием.

fn sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}
fn gated(filter: f64, gate: f64) -> f64 {
    filter.tanh() * sigmoid(gate)
}
fn main() {
    let filter = 1.5;
    let open = gated(filter, 5.0);
    let closed = gated(filter, -5.0);
    assert!(open > closed);
    assert!(closed >= 0.0);
    println!("закрытый gate={closed:.4}; открытый gate={open:.4}");
    visualize(filter);
}

fn visualize(filter: f64) {
    let points: Vec<_> = (-50..=50)
        .map(|i| {
            let gate = i as f64 / 10.0;
            (gate, gated(filter, gate))
        })
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "gate",
        "Управляемая активация",
        "gate",
        "выход",
        &[lesson_visualization::Series {
            name: "tanh(filter)*sigmoid(gate)",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
