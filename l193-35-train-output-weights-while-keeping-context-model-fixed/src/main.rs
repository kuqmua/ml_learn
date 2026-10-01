// Урок 35.8. Обучение выходных весов при неизменной модели построения контекста.
// Связь с принятой терминологией: Обучение выходной головы учебной GPT при замороженном decoder.
// Зачем здесь эта тема: После проверки decoder можно отдельно убедиться, что его выходная голова
//   обучается.
// Почему код устроен так: Замораживаем decoder и меняем только веса readout, наблюдая уменьшение
//   ошибки.
// Представь: Если скрытые состояния уже фиксированы, можно обучить только последний слой, который
//   читает их.
// Фиксируем decoder и подгоняем только выходные веса на train; качество проверяем отдельно.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
use l191_35_calculate_text_context_vectors_by_adding_position_and_weighted_past_context::calculate_text_context_vectors_by_adding_position_and_weighted_past_context;

fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
    input_value: f64,
) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
/// Бинарная перекрёстная энтропия: из последнего контекстного вектора получаем вероятность p и считаем −y·ln(p)−(1−y)·ln(1−p).
fn calculate_binary_prediction_loss_as_negative_log_target_probability_from_final_context(
    weight: &[f64; 2],
    sample: (&[usize], f64),
) -> f64 {
    let final_hidden_state: [f64; 2] =
        *calculate_text_context_vectors_by_adding_position_and_weighted_past_context(sample.0)
            .last()
            .unwrap();
    let raw_model_score: f64 =
        weight[0] * final_hidden_state[0] + weight[1] * final_hidden_state[1];
    let probability: f64 =
        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
            raw_model_score,
        )
        .clamp(1e-12, 1.0 - 1e-12);
    -sample.1 * probability.ln() - (1.0 - sample.1) * (1.0 - probability).ln()
}
fn main() {
    let validation: [(&[usize], f64); 2] = [(&[0, 0][..], 1.0), (&[1, 1][..], 0.0)];
    let mut weight: [f64; 2] = [0.0; 2];
    let baseline: f64 = validation
        .iter()
        .map(|&sample| {
            calculate_binary_prediction_loss_as_negative_log_target_probability_from_final_context(
                &weight, sample,
            )
        })
        .sum::<f64>()
        / validation.len() as f64;
    let training_data: [(&[usize], f64); 2] = [(&[0][..], 1.0), (&[1][..], 0.0)];
    for _ in 0..100 {
        let mut rate_of_change: [f64; 2] = [0.0; 2];
        for &(text_unit_identifiers, target) in &training_data {
            let final_hidden_state: [f64; 2] =
                *calculate_text_context_vectors_by_adding_position_and_weighted_past_context(
                    text_unit_identifiers,
                )
                .last()
                .unwrap();
            let error: f64 =
                calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
                    weight[0] * final_hidden_state[0] + weight[1] * final_hidden_state[1],
                ) - target;
            rate_of_change[0] += error * final_hidden_state[0];
            rate_of_change[1] += error * final_hidden_state[1];
        }
        for step_index in 0..2 {
            weight[step_index] -= 0.2 * rate_of_change[step_index] / training_data.len() as f64;
        }
    }
    let held_out: f64 = validation
        .iter()
        .map(|&sample| {
            calculate_binary_prediction_loss_as_negative_log_target_probability_from_final_context(
                &weight, sample,
            )
        })
        .sum::<f64>()
        / validation.len() as f64;
    assert!(held_out < baseline);
}
