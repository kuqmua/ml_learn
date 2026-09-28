// Урок 40.5. Потери при предсказании следующего токена.
// Логиты позиции t оцениваются по целевому токену позиции t+1.

use part_203_lesson_40_causal_self_attention::softmax;
fn cross_entropy(logits: &[f64], target: usize) -> f64 {
    -softmax(logits)[target].ln()
}
fn main() {
    // BOS, A, B, EOS: на последней позиции нет следующей цели.
    let ids = [0, 1, 2, 3];
    let logits = [
        [0.2, 2.0, 0.1, 0.0],
        [0.1, 0.2, 2.0, 0.0],
        [0.0, 0.1, 0.2, 2.0],
    ];
    let losses: Vec<f64> = (0..ids.len() - 1)
        .map(|i| cross_entropy(&logits[i], ids[i + 1]))
        .collect();
    let average = losses.iter().sum::<f64>() / losses.len() as f64;
    let wrong = cross_entropy(&[2.0, 0.2, 0.1, 0.0], ids[1]);
    assert!(average < wrong);
    println!("loss по позициям: {losses:?}; средний loss={average:.3}");
    visualize(&losses);
}
fn visualize(losses: &[f64]) {
    let points: Vec<_> = losses
        .iter()
        .enumerate()
        .map(|(i, &loss)| (i as f64, loss))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "next-token-loss",
        "Потери по позициям",
        "позиция",
        "cross entropy",
        &[lesson_visualization::Series {
            name: "loss",
            points: &points,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", path.display());
}
