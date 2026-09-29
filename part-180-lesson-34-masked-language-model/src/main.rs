// Урок 34.3. Маскированное языковое моделирование.
// Цель содержит только скрытые позиции, а encoder видит левый и правый контекст.

fn main() {
    // Три токена A, B, C; средний заменяем отдельным MASK embedding.
    let original = [0, 1, 2];
    // Плотное числовое представление объекта называют embedding.
    let dense_numeric_representations = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [0.0, 0.0]];
    let visible = [
        dense_numeric_representations[original[0]],
        dense_numeric_representations[3],
        dense_numeric_representations[original[2]],
    ];
    let context =
        part_178_lesson_34_bidirectional_encoder::bidirectional_attention(&visible, &[true; 3])
            .unwrap();
    // Скрытие позиции для её предсказания называют masked language modeling.
    let hidden_text_unit_identifier = original[1];
    // Оценку модели до преобразования в вероятность называют logit.
    let raw_model_scores: Vec<f64> = dense_numeric_representations[..3]
        .iter()
        .map(|candidate| context[1][0] * candidate[0] + context[1][1] * candidate[1])
        .collect();
    let maximum_value = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let logarithm_of_sum_of_exponentials = maximum_value
        + raw_model_scores
            .iter()
            .map(|value| (value - maximum_value).exp())
            .sum::<f64>()
            .ln();
    let loss = logarithm_of_sum_of_exponentials - raw_model_scores[hidden_text_unit_identifier];
    assert!(loss.is_finite());
    println!("цель скрытой позиции={hidden_text_unit_identifier}; MLM loss={loss:.4}");
    // Фиксированные embeddings иллюстрируют loss, а не обученный BERT.
}
