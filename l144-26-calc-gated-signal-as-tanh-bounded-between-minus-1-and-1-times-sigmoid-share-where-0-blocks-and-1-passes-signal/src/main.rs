// Урок 26.3. Управляемый выход сигнала: умножение ограниченного сигнала на долю, задаваемую второй ветвью.
// Зачем здесь эта тема: Сигналу может требоваться пропускать или подавлять найденный фильтром
//   признак.
// Почему код устроен так: Умножаем ветку tanh на сигмоидальные ворота и смотрим влияние gate.
// Представь: Если gate близок к нулю, выход ветки фильтра почти подавляется.
// Одна ветка tanh создаёт сигнал, другая sigmoid управляет его пропусканием.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.

fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(
    value: f64,
) -> f64 {
    1.0 / (1.0 + (-value).exp())
}
/// Управляемая активация WaveNet: tanh(filter)·sigmoid(gate).
fn calc_gated_signal_as_tanh_bounded_between_minus_1_and_1_times_sigmoid_share_where_0_blocks_and_1_passes_signal(
    filter: f64,
    raw_gate_score_where_0_passes_half_and_larger_values_pass_more: f64,
) -> f64 {
    calc_tanh_as_signed_signal_where_0_means_no_signal_and_large_inputs_approach_1_or_minus_1(filter)
        * calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(raw_gate_score_where_0_passes_half_and_larger_values_pass_more)
}
fn main() {
    let filter: f64 = 1.5;

    let signal_with_gate_almost_closed: f64 =
        calc_gated_signal_as_tanh_bounded_between_minus_1_and_1_times_sigmoid_share_where_0_blocks_and_1_passes_signal(
            filter, -5.0,
        );
    assert!(
        calc_gated_signal_as_tanh_bounded_between_minus_1_and_1_times_sigmoid_share_where_0_blocks_and_1_passes_signal(
            filter, 5.0,
        ) > signal_with_gate_almost_closed
    );
    assert!(signal_with_gate_almost_closed >= 0.0);

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
            name: "ограниченный сигнал × доля пропускания",
            points: &(-50..=50)
        .map(|plot_step_index| {
            let raw_gate_score_where_0_passes_half_and_larger_values_pass_more: f64 = plot_step_index as f64 / 10.0;
            (
                raw_gate_score_where_0_passes_half_and_larger_values_pass_more,
                calc_gated_signal_as_tanh_bounded_between_minus_1_and_1_times_sigmoid_share_where_0_blocks_and_1_passes_signal(
                    filter, raw_gate_score_where_0_passes_half_and_larger_values_pass_more,
                ),
            )
        })
        .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calc_tanh_as_signed_signal_where_0_means_no_signal_and_large_inputs_approach_1_or_minus_1(
    input: f64,
) -> f64 {
    input.tanh()
}
