// Урок 35.7. Обучение выходной головы tiny GPT.
// Фиксируем decoder и подгоняем только выходные веса на train; качество проверяем отдельно.

use part_186_lesson_35_tiny_gpt_forward::hidden_states;
fn sigmoid(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
fn loss(weight: &[f64; 2], sample: (&[usize], f64)) -> f64 {
    let final_hidden_state = *hidden_states(sample.0).last().unwrap();
    let logit = weight[0] * final_hidden_state[0] + weight[1] * final_hidden_state[1];
    let probability = sigmoid(logit).clamp(1e-12, 1.0 - 1e-12);
    -sample.1 * probability.ln() - (1.0 - sample.1) * (1.0 - probability).ln()
}
fn main() {
    let training_data = [(&[0][..], 1.0), (&[1][..], 0.0)];
    let validation = [(&[0, 0][..], 1.0), (&[1, 1][..], 0.0)];
    let mut weight = [0.0; 2];
    let baseline = validation
        .iter()
        .map(|&sample| loss(&weight, sample))
        .sum::<f64>()
        / validation.len() as f64;
    for _ in 0..100 {
        let mut gradient = [0.0; 2];
        for &(token_identifiers, target) in &training_data {
            let final_hidden_state = *hidden_states(token_identifiers).last().unwrap();
            let error =
                sigmoid(weight[0] * final_hidden_state[0] + weight[1] * final_hidden_state[1])
                    - target;
            gradient[0] += error * final_hidden_state[0];
            gradient[1] += error * final_hidden_state[1];
        }
        for step_index in 0..2 {
            weight[step_index] -= 0.2 * gradient[step_index] / training_data.len() as f64;
        }
    }
    let held_out = validation
        .iter()
        .map(|&sample| loss(&weight, sample))
        .sum::<f64>()
        / validation.len() as f64;
    assert!(held_out < baseline);
    println!("validation cross entropy: baseline={baseline:.3}, обученная голова={held_out:.3}");
    // Здесь обучается только readout, не все параметры GPT.
}
