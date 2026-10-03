// Урок 26.5. Прогноз следующего значения звука только по предыдущим значениям.
// Зачем здесь эта тема: Авторегрессионная модель звука объединяет причинность, широкую историю и
//   прогноз следующего отсчёта.
// Почему код устроен так: Делаем маленький forward, где каждый выход зависит только от разрешённого
//   прошлого.
// Представь: Чтобы предсказать следующий звуковой отсчёт, используем только уже известные отсчёты.
// Сочетаем причинные дилатированные свёртки, gate и вероятность следующего дискретного отсчёта.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
use l142_26_calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values::calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values;

fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}

// Текущий вход содержит только уже известные отсчёты.
fn calc_probability_of_next_sound_sample_from_history(history: &[u8]) -> f64 {
    let input: Vec<f64> = history
        .iter()
        .map(|&sample| f64::from(sample) * 2.0 - 1.0)
        .collect();

    let first_layer_bounded_signals_after_gating: Vec<f64> =
        calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &input, 0.8, 0.4, 1,
        )
        .unwrap()
        .iter()
        .zip(
            &calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
                &input, 0.2, -0.3, 1,
            )
            .unwrap(),
        )
        .map(|(&filter_value, &gate_score_controlling_signal_share)| {
            calc_tanh(filter_value)
                * calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                    gate_score_controlling_signal_share,
                )
        })
        .collect();

    let last: usize = history.len() - 1;
    let second_layer_bounded_signal_after_gating: f64 = calc_tanh(
        calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &first_layer_bounded_signals_after_gating,
            1.0,
            0.5,
            2,
        )
        .unwrap()[last],
    )
        * calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
            calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
                &first_layer_bounded_signals_after_gating,
                0.1,
                0.6,
                2,
            )
            .unwrap()[last],
        );
    calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
        2.0 * second_layer_bounded_signal_after_gating,
    )
}
fn main() {
    let mut samples: Vec<u8> = vec![1, 0, 1, 1];
    for _ in 0..4 {
        samples.push(u8::from(
            calc_probability_of_next_sound_sample_from_history(&samples) >= 0.5,
        ));
    }
    assert_eq!(samples.len(), 8);

    plot_generated_discrete_sound_values(&samples);
}

fn plot_generated_discrete_sound_values(samples: &[u8]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "samples",
        "Дискретные отсчёты",
        "t",
        "значение",
        &[lesson_visualization::Series {
            name: "отсчёт",
            points: &samples
                .iter()
                .enumerate()
                .map(|(item_index, &horizontal_value)| {
                    (item_index as f64, f64::from(horizontal_value))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calc_tanh(input: f64) -> f64 {
    input.tanh()
}
