// Урок 35.7. Обучение выходной головы tiny GPT.
// Фиксируем decoder и подгоняем только выходные веса на train; качество проверяем отдельно.

use part_186_lesson_35_tiny_gpt_forward::hidden_states;
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
fn loss(weight: &[f64; 2], sample: (&[usize], f64)) -> f64 {
    let h = *hidden_states(sample.0).last().unwrap();
    let logit = weight[0] * h[0] + weight[1] * h[1];
    let p = sigmoid(logit).clamp(1e-12, 1.0 - 1e-12);
    -sample.1 * p.ln() - (1.0 - sample.1) * (1.0 - p).ln()
}
fn main() {
    let train = [(&[0][..], 1.0), (&[1][..], 0.0)];
    let validation = [(&[0, 0][..], 1.0), (&[1, 1][..], 0.0)];
    let mut weight = [0.0; 2];
    let baseline = validation
        .iter()
        .map(|&sample| loss(&weight, sample))
        .sum::<f64>()
        / validation.len() as f64;
    for _ in 0..100 {
        let mut gradient = [0.0; 2];
        for &(ids, target) in &train {
            let h = *hidden_states(ids).last().unwrap();
            let error = sigmoid(weight[0] * h[0] + weight[1] * h[1]) - target;
            gradient[0] += error * h[0];
            gradient[1] += error * h[1];
        }
        for i in 0..2 {
            weight[i] -= 0.2 * gradient[i] / train.len() as f64;
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
