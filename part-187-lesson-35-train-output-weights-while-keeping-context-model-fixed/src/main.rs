// Урок 35.7. Обучение выходных весов при неизменной модели построения контекста.
// Связь с принятой терминологией: Обучение выходной головы учебной GPT при замороженном decoder.
// Зачем здесь эта тема: После проверки decoder можно отдельно убедиться, что его выходная голова
//   обучается.
// Почему код устроен так: Замораживаем decoder и меняем только веса readout, наблюдая уменьшение
//   ошибки.
// Представь: Если скрытые состояния уже фиксированы, можно обучить только последний слой, который
//   читает их.
// Фиксируем decoder и подгоняем только выходные веса на train; качество проверяем отдельно.

/// Сигмоида: 1 / (1 + e^(−score)); превращает оценку модели в число от 0 до 1.
fn one_divided_by_one_plus_e_to_negative_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
/// Бинарная перекрёстная энтропия: из последнего контекстного вектора получаем вероятность p и считаем −y·ln(p)−(1−y)·ln(1−p).
fn negative_log_probability_of_binary_label_from_final_context(
    weight: &[f64; 2],
    sample: (&[usize], f64),
) -> f64 {
    let final_hidden_state: [f64; 2] = *part_186_lesson_35_convert_text_identifiers_to_context_and_next_piece_scores::add_position_to_text_vectors_then_add_weighted_past_context(sample.0)
        .last()
        .unwrap();
    lesson_trace::trace_step!(final_hidden_state);
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_score: f64 =
        weight[0] * final_hidden_state[0] + weight[1] * final_hidden_state[1];
    lesson_trace::trace_step!(raw_model_score);
    // Ограничиваем p интервалом [10⁻¹², 1−10⁻¹²], чтобы ln(p) и ln(1−p) были конечными.
    let probability: f64 =
        one_divided_by_one_plus_e_to_negative_score(raw_model_score).clamp(1e-12, 1.0 - 1e-12);
    lesson_trace::trace_step!(probability);
    -sample.1 * probability.ln() - (1.0 - sample.1) * (1.0 - probability).ln()
}
fn main() {
    lesson_trace::enable();
    let training_data: [(&[usize], f64); 2] = [(&[0][..], 1.0), (&[1][..], 0.0)];
    lesson_trace::trace_step!(training_data);
    let validation: [(&[usize], f64); 2] = [(&[0, 0][..], 1.0), (&[1, 1][..], 0.0)];
    lesson_trace::trace_step!(validation);
    let mut weight: [f64; 2] = [0.0; 2];
    lesson_trace::trace_step!(weight);
    let baseline: f64 = validation
        .iter()
        .map(|&sample| negative_log_probability_of_binary_label_from_final_context(&weight, sample))
        .sum::<f64>()
        / validation.len() as f64;
    lesson_trace::trace_step!(baseline);
    // Обновляем только обучаемый выходной слой 100 раз; декодер в этом опыте заморожен.
    // Число шагов ограничивает учебное обучение и позволяет затем сравнить ошибку.
    for _ in 0..100 {
        // Производную функции по параметру или вектор таких производных называют gradient.
        let mut rate_of_change: [f64; 2] = [0.0; 2];
        lesson_trace::trace_step!(rate_of_change);
        // Единицу текста, которую модель обрабатывает как одно целое, называют token.
        for &(text_unit_identifiers, target) in &training_data {
            lesson_trace::trace_step!(text_unit_identifiers);
            lesson_trace::trace_step!(target);
            let final_hidden_state: [f64; 2] = *part_186_lesson_35_convert_text_identifiers_to_context_and_next_piece_scores::add_position_to_text_vectors_then_add_weighted_past_context(text_unit_identifiers)
                .last()
                .unwrap();
            lesson_trace::trace_step!(final_hidden_state);
            let error: f64 = one_divided_by_one_plus_e_to_negative_score(
                weight[0] * final_hidden_state[0] + weight[1] * final_hidden_state[1],
            ) - target;
            lesson_trace::trace_step!(error);
            rate_of_change[0] += error * final_hidden_state[0];
            lesson_trace::trace_step!(rate_of_change);
            rate_of_change[1] += error * final_hidden_state[1];
            lesson_trace::trace_step!(rate_of_change);
        }
        for step_index in 0..2 {
            lesson_trace::trace_step!(step_index);
            weight[step_index] -= 0.2 * rate_of_change[step_index] / training_data.len() as f64;
            lesson_trace::trace_step!(weight);
        }
    }
    let held_out: f64 = validation
        .iter()
        .map(|&sample| negative_log_probability_of_binary_label_from_final_context(&weight, sample))
        .sum::<f64>()
        / validation.len() as f64;
    lesson_trace::trace_step!(held_out);
    assert!(held_out < baseline);
    println!("validation cross entropy: baseline={baseline:.3}, обученная голова={held_out:.3}");
    // Здесь обучается только readout, не все параметры GPT.
}
