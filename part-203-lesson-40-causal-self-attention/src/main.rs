// Урок 40.2. Причинное self-attention.
// Для позиции i softmax вычисляется только по позициям 0..=i.

use part_203_lesson_40_causal_self_attention::causal_attention;
fn main() {
    let states = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let context = causal_attention(&states, &states, &states).unwrap();
    assert_eq!(context[0], states[0]);
    visualize(&states);
    for (index, state) in context.iter().enumerate() {
        println!("позиция {index}: {state:?}");
    }
}

fn visualize(states: &[[f64; 2]]) {
    use part_203_lesson_40_causal_self_attention::softmax;
    let matrix: Vec<Vec<f64>> = states
        .iter()
        .enumerate()
        .map(|(i, q)| {
            let logits: Vec<_> = (0..=i)
                .map(|j| (q[0] * states[j][0] + q[1] * states[j][1]) / 2.0_f64.sqrt())
                .collect();
            let weights = softmax(&logits);
            (0..states.len())
                .map(|j| if j <= i { weights[j] } else { 0.0 })
                .collect()
        })
        .collect();
    let path = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "causal-attention",
        "Веса причинного внимания",
        &matrix,
    )
    .expect("график");
    println!("график: {}", path.display());
}
