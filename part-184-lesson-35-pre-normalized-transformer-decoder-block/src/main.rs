// Урок 35.4. Блок Transformer decoder с нормализацией перед подслоями.
// Зачем здесь эта тема: Генеративный блок должен повторяемо соединять нормализацию, причинное
//   внимание и feed-forward.
// Почему код устроен так: Показываем pre-norm и residual после каждого подслоя на коротком
//   префиксе.
// Представь: Каждый подслой получает нормированный вход, а residual возвращает его в итог этого
//   шага.
// Нормализация, причинное внимание, residual, FFN и второй residual образуют блок.

fn layer_normalize_token_vector(input_value: [f64; 2]) -> [f64; 2] {
    // Среднее и дисперсию считаем по двум координатам токена; 10⁻⁵ не даёт делить на ноль.
    let mean: f64 = (input_value[0] + input_value[1]) / 2.0;
    lesson_trace::trace_step!(mean);
    let variance: f64 = ((input_value[0] - mean).powi(2) + (input_value[1] - mean).powi(2)) / 2.0;
    lesson_trace::trace_step!(variance);
    [
        (input_value[0] - mean) / (variance + 1e-5).sqrt(),
        (input_value[1] - mean) / (variance + 1e-5).sqrt(),
    ]
}
fn apply_transformer_decoder_block(input: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let normalized: Vec<[f64; 2]> = input
        .iter()
        .copied()
        .map(layer_normalize_token_vector)
        .collect();
    lesson_trace::trace_step!(normalized);
    let attention: Vec<[f64; 2]> = part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::causal_self_attention_over_query_key_value_sequences(
        &normalized,
        &normalized,
        &normalized,
    )
    .unwrap();
    lesson_trace::trace_step!(attention);
    input
        .iter()
        .zip(&attention)
        .map(|(&original, &context)| {
            // Добавление входа блока к его преобразованному выходу называют residual connection.
            let input_plus_transformed_value: [f64; 2] =
                [original[0] + context[0], original[1] + context[1]];
            lesson_trace::trace_step!(input_plus_transformed_value);
            let norm: [f64; 2] = layer_normalize_token_vector(input_plus_transformed_value);
            lesson_trace::trace_step!(norm);
            // Упрощённый FFN: два ReLU-канала и фиксированная выходная проекция.
            [
                input_plus_transformed_value[0] + 0.2 * norm[0].max(0.0),
                input_plus_transformed_value[1] + 0.2 * norm[1].max(0.0),
            ]
        })
        .collect()
}
fn main() {
    lesson_trace::enable();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    lesson_trace::trace_step!(states);
    let output: Vec<[f64; 2]> = apply_transformer_decoder_block(&states);
    lesson_trace::trace_step!(output);
    assert_eq!(output.len(), states.len());
    println!("после decoder block: {output:?}");
}
