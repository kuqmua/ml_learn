// Урок 35.7. Обучение выходной головы учебной GPT при замороженном decoder.
// Фиксируем decoder и подгоняем только выходные веса на train; качество проверяем отдельно.

fn sigmoid_activation_of_raw_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
fn binary_cross_entropy_for_gpt_readout_prediction(
    weight: &[f64; 2],
    sample: (&[usize], f64),
) -> f64 {
    let final_hidden_state: [f64; 2] = *part_186_lesson_35_calculate_tiny_gpt_output_logits_from_token_ids::calculate_decoder_hidden_states_for_token_ids(sample.0)
        .last()
        .unwrap();
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_score: f64 =
        weight[0] * final_hidden_state[0] + weight[1] * final_hidden_state[1];
    let probability: f64 =
        sigmoid_activation_of_raw_score(raw_model_score).clamp(1e-12, 1.0 - 1e-12);
    -sample.1 * probability.ln() - (1.0 - sample.1) * (1.0 - probability).ln()
}
fn main() {
    let training_data: [(&[usize], f64); 2] = [(&[0][..], 1.0), (&[1][..], 0.0)];
    let validation: [(&[usize], f64); 2] = [(&[0, 0][..], 1.0), (&[1, 1][..], 0.0)];
    let mut weight: [f64; 2] = [0.0; 2];
    let baseline: f64 = validation
        .iter()
        .map(|&sample| binary_cross_entropy_for_gpt_readout_prediction(&weight, sample))
        .sum::<f64>()
        / validation.len() as f64;
    for _ in 0..100 {
        // Производную функции по параметру или вектор таких производных называют gradient.
        let mut rate_of_change: [f64; 2] = [0.0; 2];
        // Единицу текста, которую модель обрабатывает как одно целое, называют token.
        for &(text_unit_identifiers, target) in &training_data {
            let final_hidden_state: [f64; 2] = *part_186_lesson_35_calculate_tiny_gpt_output_logits_from_token_ids::calculate_decoder_hidden_states_for_token_ids(text_unit_identifiers)
                .last()
                .unwrap();
            let error: f64 = sigmoid_activation_of_raw_score(
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
        .map(|&sample| binary_cross_entropy_for_gpt_readout_prediction(&weight, sample))
        .sum::<f64>()
        / validation.len() as f64;
    assert!(held_out < baseline);
    println!("validation cross entropy: baseline={baseline:.3}, обученная голова={held_out:.3}");
    // Здесь обучается только readout, не все параметры GPT.
}
