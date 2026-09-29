// Урок 36.8. Блок по мотивам Qwen с RMSNorm, RoPE, GQA и SwiGLU.
// Собираем pre-RMSNorm, QK-Norm, RoPE, GQA, residual и SwiGLU без реальных весов Qwen.

fn apply_qwen_inspired_transformer_block(states: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let gamma = [1.0, 1.0];
    let norm: Vec<[f64; 2]> = states
        .iter()
        .map(|input_value| {
            let second_input_value =
                part_188_lesson_36_normalize_token_vector_by_root_mean_square::normalize_vector_by_root_mean_square(
                    input_value,
                    &gamma,
                    1e-6,
                )
                .unwrap();
            [second_input_value[0], second_input_value[1]]
        })
        .collect();
    // Одна K/V-голова хранит общие ключи и значения для двух Q-голов.
    let keys: Vec<_> = norm
        .iter()
        .enumerate()
        .map(|(position, input_value)| {
            let key =
                part_188_lesson_36_normalize_token_vector_by_root_mean_square::normalize_vector_by_root_mean_square(
                    input_value,
                    &gamma,
                    1e-6,
                )
                .unwrap();
            part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
                [key[0], key[1]],
                position,
                0.1,
            )
        })
        .collect();
    let mut output = Vec::new();
    for index in 0..states.len() {
        let mut context = [0.0; 2];
        for head in 0..2 {
            // Разные проекции Q обращаются к одним и тем же сохранённым K/V.
            let raw_query = if head == 0 {
                norm[index]
            } else {
                [norm[index][1], -norm[index][0]]
            };
            let query =
                part_188_lesson_36_normalize_token_vector_by_root_mean_square::normalize_vector_by_root_mean_square(
                    &raw_query, &gamma, 1e-6,
                )
                .unwrap();
            let query =
                part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_position::rotate_vector_coordinate_pair_by_token_position(
                    [query[0], query[1]],
                    index,
                    0.1,
                );
            // Оценку модели до преобразования в вероятность называют logit.
            let raw_model_scores: Vec<f64> = (0..=index)
                .map(|past| (query[0] * keys[past][0] + query[1] * keys[past][1]) / 2.0_f64.sqrt())
                .collect();
            let weights = part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::softmax_probabilities_from_raw_model_scores(&raw_model_scores);
            for (past, &weight) in weights.iter().enumerate() {
                context[0] += 0.5 * weight * norm[past][0];
                context[1] += 0.5 * weight * norm[past][1];
            }
        }
        // Добавление входа блока к его преобразованному выходу называют residual connection.
        let input_plus_transformed_value =
            [states[index][0] + context[0], states[index][1] + context[1]];
        let feed_forward_input =
            part_188_lesson_36_normalize_token_vector_by_root_mean_square::normalize_vector_by_root_mean_square(
                &input_plus_transformed_value,
                &gamma,
                1e-6,
            )
            .unwrap();
        output.push([
            input_plus_transformed_value[0]
                + 0.1
                    * part_192_lesson_36_apply_swiglu_gate_and_up_projection_in_feed_forward_layer::swish_gated_linear_unit_of_gate_and_up_projection(
                        feed_forward_input[0],
                        feed_forward_input[1],
                    ),
            input_plus_transformed_value[1]
                + 0.1
                    * part_192_lesson_36_apply_swiglu_gate_and_up_projection_in_feed_forward_layer::swish_gated_linear_unit_of_gate_and_up_projection(
                        feed_forward_input[1],
                        feed_forward_input[0],
                    ),
        ]);
    }
    output
}
fn main() {
    let states = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let output = apply_qwen_inspired_transformer_block(&states);
    assert_eq!(
        apply_qwen_inspired_transformer_block(&states[..1])[0],
        output[0]
    );
    println!("выход учебного блока: {output:?}");
    // Реальный Qwen3 имеет многомерные проекции, обученные веса и масштабные данные.
}
