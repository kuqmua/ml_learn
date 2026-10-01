// Урок 26.3. Управляемый выход сигнала: умножение ограниченного сигнала на долю, задаваемую второй ветвью.
// Связь с принятой терминологией: Управляемая активация с ветками tanh и sigmoid.
// Зачем здесь эта тема: Сигналу может требоваться пропускать или подавлять найденный фильтром
//   признак.
// Почему код устроен так: Умножаем ветку tanh на сигмоидальные ворота и смотрим влияние gate.
// Представь: Если gate близок к нулю, выход ветки фильтра почти подавляется.
// Одна ветка tanh создаёт сигнал, другая sigmoid управляет его пропусканием.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.

fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
    value: f64,
) -> f64 {
    1.0 / (1.0 + (-value).exp())
}
/// Управляемая активация WaveNet: tanh(filter)·sigmoid(gate).
fn calculate_gated_signal_as_tanh_of_filter_times_one_over_one_plus_e_to_negative_gate(
    filter: f64,
    gate: f64,
) -> f64 {
    filter.tanh()
        * calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(gate)
}
fn main() {
    let filter: f64 = 1.5;

    let closed: f64 =
        calculate_gated_signal_as_tanh_of_filter_times_one_over_one_plus_e_to_negative_gate(
            filter, -5.0,
        );
    assert!(
        calculate_gated_signal_as_tanh_of_filter_times_one_over_one_plus_e_to_negative_gate(
            filter, 5.0,
        ) > closed
    );
    assert!(closed >= 0.0);

    plot_filter_output_multiplied_by_fraction_controlled_by_gate(filter);
}

fn plot_filter_output_multiplied_by_fraction_controlled_by_gate(filter: f64) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "gate",
        "Управляемая активация",
        "gate",
        "выход",
        &[lesson_visualization::Series {
            name: "tanh(filter)*calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(gate)",
            points: &(-50..=50)
        .map(|plot_step_index| {
            let gate: f64 = plot_step_index as f64 / 10.0;
            (
                gate,
                calculate_gated_signal_as_tanh_of_filter_times_one_over_one_plus_e_to_negative_gate(
                    filter, gate,
                ),
            )
        })
        .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
