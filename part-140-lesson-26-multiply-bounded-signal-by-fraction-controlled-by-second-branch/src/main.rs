// Урок 26.3. Умножение ограниченного сигнала на долю, задаваемую второй ветвью.
// Связь с принятой терминологией: Управляемая активация с ветками tanh и sigmoid.
// Зачем здесь эта тема: Сигналу может требоваться пропускать или подавлять найденный фильтром
//   признак.
// Почему код устроен так: Умножаем ветку tanh на сигмоидальные ворота и смотрим влияние gate.
// Представь: Если gate близок к нулю, выход ветки фильтра почти подавляется.
// Одна ветка tanh создаёт сигнал, другая sigmoid управляет его пропусканием.

/// Сигмоида: 1 / (1 + e^(−score)); превращает оценку модели в число от 0 до 1.
fn one_divided_by_one_plus_e_to_negative_score(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}
/// Управляемая активация WaveNet: tanh(filter)·sigmoid(gate).
fn multiply_tanh_of_filter_by_one_over_one_plus_e_to_negative_gate(filter: f64, gate: f64) -> f64 {
    filter.tanh() * one_divided_by_one_plus_e_to_negative_score(gate)
}
fn main() {
    lesson_trace::enable();
    let filter: f64 = 1.5;
    lesson_trace::trace_step!(filter);
    let open: f64 = multiply_tanh_of_filter_by_one_over_one_plus_e_to_negative_gate(filter, 5.0);
    lesson_trace::trace_step!(open);
    let closed: f64 = multiply_tanh_of_filter_by_one_over_one_plus_e_to_negative_gate(filter, -5.0);
    lesson_trace::trace_step!(closed);
    assert!(open > closed);
    assert!(closed >= 0.0);
    println!("закрытый gate={closed:.4}; открытый gate={open:.4}");
    lesson_trace::disable();
    plot_filter_output_multiplied_by_fraction_controlled_by_gate(filter);
}

fn plot_filter_output_multiplied_by_fraction_controlled_by_gate(filter: f64) {
    let points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            let gate: f64 = plot_step_index as f64 / 10.0;
            (
                gate,
                multiply_tanh_of_filter_by_one_over_one_plus_e_to_negative_gate(filter, gate),
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
            name: "tanh(filter)*one_divided_by_one_plus_e_to_negative_score(gate)",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
