// Урок 26.5. Прогноз следующего значения звука только по предыдущим значениям.
// Связь с принятой терминологией: Авторегрессионная модель звука с дилатированными причинными свёртками.
// Зачем здесь эта тема: Авторегрессионная модель звука объединяет причинность, широкую историю и
//   прогноз следующего отсчёта.
// Почему код устроен так: Делаем маленький forward, где каждый выход зависит только от разрешённого
//   прошлого.
// Представь: Чтобы предсказать следующий звуковой отсчёт, используем только уже известные отсчёты.
// Сочетаем причинные дилатированные свёртки, gate и вероятность следующего дискретного отсчёта.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
    input_value: f64,
) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}

// Текущий вход содержит только уже известные отсчёты.
fn calculate_probability_of_next_sound_sample_from_history(history: &[u8]) -> f64 {
    let input: Vec<f64> = history
        .iter()
        .map(|&sample| f64::from(sample) * 2.0 - 1.0)
        .collect();
    lesson_trace::trace_step!(input);
    let filter_one: Vec<f64> =
        l142_26_calculate_causal_filter_output_by_summing_weighted_current_and_past_values::calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &input, 0.8, 0.4, 1,
        )
        .unwrap();
    lesson_trace::trace_step!(filter_one);
    let gate_one: Vec<f64> =
        l142_26_calculate_causal_filter_output_by_summing_weighted_current_and_past_values::calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &input, 0.2, -0.3, 1,
        )
        .unwrap();
    lesson_trace::trace_step!(gate_one);
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    let layer_one: Vec<f64> = filter_one
        .iter()
        .zip(&gate_one)
        .map(|(&filter_value, &rate_of_change_value)| {
            filter_value.tanh()
                * calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                    rate_of_change_value,
                )
        })
        .collect();
    lesson_trace::trace_step!(layer_one);
    let filter_two: Vec<f64> =
        l142_26_calculate_causal_filter_output_by_summing_weighted_current_and_past_values::calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &layer_one, 1.0, 0.5, 2,
        )
        .unwrap();
    lesson_trace::trace_step!(filter_two);
    let gate_two: Vec<f64> =
        l142_26_calculate_causal_filter_output_by_summing_weighted_current_and_past_values::calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &layer_one, 0.1, 0.6, 2,
        )
        .unwrap();
    lesson_trace::trace_step!(gate_two);
    let last: usize = history.len() - 1;
    lesson_trace::trace_step!(last);
    let output: f64 = filter_two[last].tanh()
        * calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
            gate_two[last],
        );
    lesson_trace::trace_step!(output);
    calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(2.0 * output)
}
fn main() {
    lesson_trace::enable();
    let mut samples: Vec<u8> = vec![1, 0, 1, 1];
    lesson_trace::trace_step!(samples);
    lesson_trace::trace_note!(
        "Начальная история содержит четыре отсчёта; генерируем ещё четыре для короткого примера."
    );
    for _ in 0..4 {
        let probability: f64 = calculate_probability_of_next_sound_sample_from_history(&samples);
        lesson_trace::trace_step!(probability);
        lesson_trace::trace_note!(
            "0.5 — порог бинарного решения: вероятность не ниже половины даёт отсчёт 1."
        );
        let next: u8 = u8::from(probability >= 0.5);
        lesson_trace::trace_step!(next);
        samples.push(next);
        println!("P(следующий отсчёт=1)={probability:.3}; выбор={next}");
    }
    assert_eq!(samples.len(), 8);
    lesson_trace::trace_note!(
        "Фиксированные веса здесь показывают только прямой проход; обучение остаётся отдельной задачей."
    );
    println!("дискретный звук: {samples:?}");
    lesson_trace::disable();
    plot_generated_discrete_sound_values(&samples);
}

fn plot_generated_discrete_sound_values(samples: &[u8]) {
    let points: Vec<(f64, f64)> = samples
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, f64::from(horizontal_value)))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "samples",
        "Дискретные отсчёты",
        "t",
        "значение",
        &[lesson_visualization::Series {
            name: "отсчёт",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
