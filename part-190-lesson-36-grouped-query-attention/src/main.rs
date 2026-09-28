// Урок 36.3. Grouped-query attention.
// Несколько Q-голов совместно используют меньшее число K/V-голов.

use part_182_lesson_35_causal_self_attention::softmax;
fn main() {
    // Четырём Q-головам соответствуют две K/V-головы.
    let queries = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [-1.0, 1.0]];
    let keys = [[[1.0, 0.0], [0.0, 1.0]], [[0.0, 1.0], [1.0, 0.0]]];
    let values = [[[1.0, 0.0], [0.0, 1.0]], [[0.2, 0.8], [0.8, 0.2]]];
    let mut output = Vec::new();
    for (head, &query) in queries.iter().enumerate() {
        let group = head / 2;
        let logits: Vec<_> = keys[group]
            .iter()
            .map(|k| query[0] * k[0] + query[1] * k[1])
            .collect();
        let weights = softmax(&logits);
        output.push([
            weights[0] * values[group][0][0] + weights[1] * values[group][1][0],
            weights[0] * values[group][0][1] + weights[1] * values[group][1][1],
        ]);
    }
    assert_eq!(output.len(), 4);
    println!("4 Q / 2 KV головы: {output:?}");
}
