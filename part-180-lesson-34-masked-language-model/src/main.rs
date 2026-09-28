// Урок 34.3. Маскированное языковое моделирование.
// Цель содержит только скрытые позиции, а encoder видит левый и правый контекст.

use part_178_lesson_34_bidirectional_encoder::bidirectional_attention;
fn main() {
    // Три токена A, B, C; средний заменяем отдельным MASK embedding.
    let original = [0, 1, 2];
    let embeddings = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [0.0, 0.0]];
    let visible = [
        embeddings[original[0]],
        embeddings[3],
        embeddings[original[2]],
    ];
    let context = bidirectional_attention(&visible, &[true; 3]).unwrap();
    let masked_target = original[1];
    let logits: Vec<f64> = embeddings[..3]
        .iter()
        .map(|candidate| context[1][0] * candidate[0] + context[1][1] * candidate[1])
        .collect();
    let maximum_value = logits.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let logarithm_of_sum_of_exponentials = maximum_value
        + logits
            .iter()
            .map(|value| (value - maximum_value).exp())
            .sum::<f64>()
            .ln();
    let loss = logarithm_of_sum_of_exponentials - logits[masked_target];
    assert!(loss.is_finite());
    println!("цель скрытой позиции={masked_target}; MLM loss={loss:.4}");
    // Фиксированные embeddings иллюстрируют loss, а не обученный BERT.
}
