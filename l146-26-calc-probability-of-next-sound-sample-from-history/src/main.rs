// Урок 146. Последовательно дополнять двоичный сигнал по истории, используя причинные фильтры и
// управление долями сигнала.
// Сравниваем вероятности после разных историй; это упрощённый двоичный пример, а не синтез
// полноценного звука.

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

    let layer1_bounded_signals_after_gating: Vec<f64> =
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
    let layer2_bounded_signal_after_gating: f64 = calc_tanh(
        calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &layer1_bounded_signals_after_gating,
            1.0,
            0.5,
            2,
        )
        .unwrap()[last],
    )
        * calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
            calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
                &layer1_bounded_signals_after_gating,
                0.1,
                0.6,
                2,
            )
            .unwrap()[last],
        );
    calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(2.0 * layer2_bounded_signal_after_gating)
}
fn main() {
    let mut samples: Vec<u8> = vec![1, 0, 1, 1];
    for _ in 0..4 {
        samples.push(u8::from(
            calc_probability_of_next_sound_sample_from_history(&samples) >= 0.5,
        ));
    }
    assert_eq!(samples.len(), 8);

    // Выполняем вычисления из примера.
    let _ = &samples;

    let low = calc_probability_of_next_sound_sample_from_history(&[0, 0, 0, 0]);
    let high = calc_probability_of_next_sound_sample_from_history(&[1, 1, 1, 1]);
    println!(
        "История из нулей: P(1)={low:.4}; из единиц: P(1)={high:.4}; дополненная последовательность={samples:?}"
    );
    assert!(high > low);
    assert!(low < 0.5 && high > 0.5);
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calc_tanh(input: f64) -> f64 {
    input.tanh()
}

// Чему учит этот урок:
// Учимся последовательно дополнять двоичный сигнал по истории, используя причинные фильтры и
// управление долями сигнала.
// Сравниваем вероятности после разных историй; это упрощённый двоичный пример, а не синтез
// полноценного звука.
