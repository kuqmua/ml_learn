// Урок 35.6. KV-cache при генерации.
// Сохраняем K/V прошлых токенов и сверяем последний выход с полным причинным пересчётом.

use part_177_lesson_34_causal_self_attention::{causal_attention, softmax};
fn main() {
    let states = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let full = causal_attention(&states, &states, &states).unwrap();
    let mut cached_keys = Vec::new();
    let mut cached_values = Vec::new();
    let mut cached_outputs = Vec::new();
    for &new_state in &states {
        cached_keys.push(new_state);
        cached_values.push(new_state);
        let logits: Vec<f64> = cached_keys
            .iter()
            .map(|key| (new_state[0] * key[0] + new_state[1] * key[1]) / 2.0_f64.sqrt())
            .collect();
        let weights = softmax(&logits);
        let output = weights
            .iter()
            .zip(&cached_values)
            .fold([0.0; 2], |mut out, (&w, v)| {
                out[0] += w * v[0];
                out[1] += w * v[1];
                out
            });
        cached_outputs.push(output);
    }
    for (cached, recomputed) in cached_outputs.iter().zip(full) {
        assert!((cached[0] - recomputed[0]).abs() < 1e-12);
        assert!((cached[1] - recomputed[1]).abs() < 1e-12);
    }
    println!("выходы с KV-cache: {cached_outputs:?}");
}
