// Урок 36.8. Объединение масштабирования координат, поворотов, общего контекста и управляемых ветвей.
// Связь с принятой терминологией: Блок по мотивам Qwen с RMSNorm, RoPE, GQA и SwiGLU.
// Зачем здесь эта тема: После изучения частей современного decoder нужно увидеть их порядок и
//   совместимость форм.
// Почему код устроен так: Собираем RMSNorm, RoPE, GQA и SwiGLU в один маленький блок с причинным
//   вниманием.
// Представь: Знакомые части блока соединяются в одном порядке: нормировка, внимание и
//   преобразование токена.
// Собираем pre-RMSNorm, QK-Norm, RoPE, GQA, residual и SwiGLU без реальных весов Qwen.

// Во всех вызовах RMSNorm в блоке ε=10⁻⁶ защищает от нулевого среднего квадрата координат.
/// Учебный блок по мотивам Qwen: RMSNorm, поворот координат по позиции, два набора весов внимания с общими ключами и значениями, затем SwiGLU с прибавлением входа.
fn calculate_sequence_block_output_by_normalizing_rotating_and_mixing_past_values_and_gated_features(
    states: &[[f64; 2]],
) -> Vec<[f64; 2]> {
    let gamma: [f64; 2] = [1.0, 1.0];
    lesson_trace::trace_step!(gamma);
    let norm: Vec<[f64; 2]> = states
        .iter()
        .map(|input_value| {
            let second_input_value: Vec<f64> =
                part_188_lesson_36_divide_coordinates_by_square_root_of_mean_square_and_apply_weights::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
                    input_value,
                    &gamma,
                    1e-6,
                )
                .unwrap();
            lesson_trace::trace_step!(second_input_value);
            [second_input_value[0], second_input_value[1]]
        })
        .collect();
    lesson_trace::trace_step!(norm);
    // Одна K/V-голова хранит общие ключи и значения для двух Q-голов.
    let keys: Vec<[f64; 2]> = norm
        .iter()
        .enumerate()
        .map(|(position, input_value)| {
            let key: Vec<f64> =
                part_188_lesson_36_divide_coordinates_by_square_root_of_mean_square_and_apply_weights::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
                    input_value,
                    &gamma,
                    1e-6,
                )
                .unwrap();
            lesson_trace::trace_step!(key);
            part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
                [key[0], key[1]],
                position,
                0.1,
            )
        })
        .collect();
    lesson_trace::trace_step!(keys);
    let mut output: Vec<[f64; 2]> = Vec::new();
    lesson_trace::trace_step!(output);
    for index in 0..states.len() {
        lesson_trace::trace_step!(index);
        let mut context: [f64; 2] = [0.0; 2];
        lesson_trace::trace_step!(context);
        for head in 0..2 {
            lesson_trace::trace_step!(head);
            // Разные проекции Q обращаются к одним и тем же сохранённым K/V.
            let raw_query: [f64; 2] = if head == 0 {
                norm[index]
            } else {
                [norm[index][1], -norm[index][0]]
            };
            lesson_trace::trace_step!(raw_query);
            let query: Vec<f64> =
                part_188_lesson_36_divide_coordinates_by_square_root_of_mean_square_and_apply_weights::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
                    &raw_query, &gamma, 1e-6,
                )
                .unwrap();
            lesson_trace::trace_step!(query);
            let query: [f64; 2] =
                part_189_lesson_36_rotate_query_and_key_coordinate_pairs_by_text_position::rotate_vector_coordinate_pair_by_token_position(
                    [query[0], query[1]],
                    index,
                    0.1,
                );
            lesson_trace::trace_step!(query);
            // Оценку модели до преобразования в вероятность называют logit.
            let raw_model_scores: Vec<f64> = (0..=index)
                .map(|past| (query[0] * keys[past][0] + query[1] * keys[past][1]) / 2.0_f64.sqrt())
                .collect();
            lesson_trace::trace_step!(raw_model_scores);
            let weights: Vec<f64> = part_182_lesson_35_sum_current_and_past_values_with_query_key_match_weights::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(&raw_model_scores);
            lesson_trace::trace_step!(weights);
            for (past, &weight) in weights.iter().enumerate() {
                lesson_trace::trace_step!(past);
                lesson_trace::trace_step!(weight);
                context[0] += 0.5 * weight * norm[past][0];
                lesson_trace::trace_step!(context);
                context[1] += 0.5 * weight * norm[past][1];
                lesson_trace::trace_step!(context);
            }
        }
        // Добавление входа блока к его преобразованному выходу называют residual connection.
        let input_plus_transformed_value: [f64; 2] =
            [states[index][0] + context[0], states[index][1] + context[1]];
        lesson_trace::trace_step!(input_plus_transformed_value);
        let feed_forward_input: Vec<f64> =
            part_188_lesson_36_divide_coordinates_by_square_root_of_mean_square_and_apply_weights::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
                &input_plus_transformed_value,
                &gamma,
                1e-6,
            )
            .unwrap();
        lesson_trace::trace_step!(feed_forward_input);
        output.push([
            input_plus_transformed_value[0]
                + 0.1
                    * part_192_lesson_36_multiply_two_branches_and_control_output_with_smooth_gate::calculate_gated_layer_output_as_gate_times_up_value_over_one_plus_e_to_negative_gate(
                        feed_forward_input[0],
                        feed_forward_input[1],
                    ),
            input_plus_transformed_value[1]
                + 0.1
                    * part_192_lesson_36_multiply_two_branches_and_control_output_with_smooth_gate::calculate_gated_layer_output_as_gate_times_up_value_over_one_plus_e_to_negative_gate(
                        feed_forward_input[1],
                        feed_forward_input[0],
                    ),
        ]);
    }
    output
}
fn main() {
    lesson_trace::enable();
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    lesson_trace::trace_step!(states);
    let output: Vec<[f64; 2]> =
        calculate_sequence_block_output_by_normalizing_rotating_and_mixing_past_values_and_gated_features(&states);
    lesson_trace::trace_step!(output);
    assert_eq!(
        calculate_sequence_block_output_by_normalizing_rotating_and_mixing_past_values_and_gated_features(&states[..1])[0],
        output[0]
    );
    println!("выход учебного блока: {output:?}");
    // Реальный Qwen3 имеет многомерные проекции, обученные веса и масштабные данные.
}
