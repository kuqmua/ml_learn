// Урок 36.8. Выход блока последовательности: масштабирование, повороты, общий контекст и управляемые ветви.
// Зачем здесь эта тема: После изучения частей современного decoder нужно увидеть их порядок и
//   совместимость форм.
// Почему код устроен так: Собираем RMSNorm, RoPE, GQA и SwiGLU в один маленький блок с причинным
//   вниманием.
// Представь: Знакомые части блока соединяются в одном порядке: нормировка, внимание и
//   преобразование токена.
// Собираем pre-RMSNorm, QK-Norm, RoPE, GQA, прибавление входа и SwiGLU без реальных весов Qwen.

// Во всех вызовах RMSNorm в блоке ε=10⁻⁶ защищает от нулевого среднего квадрата координат.
/// Учебный блок по мотивам Qwen: RMSNorm, поворот координат по позиции, два набора весов внимания с общими ключами и значениями, затем SwiGLU с прибавлением входа.
use l186_35_calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum;
use l194_36_normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights::normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights;
use l195_36_rotate_vec_coord_pair_by_token_position::rotate_vec_coord_pair_by_token_position;
use l198_36_calc_gated_layer_output_as_silu_gate_times_up_value::calc_gated_layer_output_as_silu_gate_times_up_value;

fn calc_sequence_block_output_by_normalizing_rotating_and_mixing_past_values_and_gated_features<
    const N: usize,
>(
    states: &[[f64; 2]; N],
) -> [[f64; 2]; N] {
    let learned_coord_scales_after_root_mean_square_normalization: [f64; 2] = [1.0, 1.0];
    let scaled_states: [[f64; 2]; N] = std::array::from_fn(|index| {
        let input_value = &states[index];
        let input2_value: [f64; 2] =
            normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
                input_value,
                &learned_coord_scales_after_root_mean_square_normalization,
                1e-6,
            )
            .unwrap();
        [input2_value[0], input2_value[1]]
    });
    let keys: [[f64; 2]; N] = std::array::from_fn(|position| {
        let input_value = &scaled_states[position];
        let key: [f64; 2] =
            normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
                input_value,
                &learned_coord_scales_after_root_mean_square_normalization,
                1e-6,
            )
            .unwrap();
        rotate_vec_coord_pair_by_token_position([key[0], key[1]], position, 0.1)
    });
    std::array::from_fn(|index| {
        let mut context: [f64; 2] = [0.0; 2];
        for head in 0..2 {
            let raw_query: [f64; 2] = if head == 0 {
                scaled_states[index]
            } else {
                [scaled_states[index][1], -scaled_states[index][0]]
            };
            let query: [f64; 2] =
                normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
                    &raw_query,
                    &learned_coord_scales_after_root_mean_square_normalization,
                    1e-6,
                )
                .unwrap();
            let query: [f64; 2] =
                rotate_vec_coord_pair_by_token_position([query[0], query[1]], index, 0.1);

            for (past, weight) in calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
                &(0..=index)
                    .map(|past| {
                        (query[0] * keys[past][0] + query[1] * keys[past][1])
                            / 2.0_f64.sqrt()
                    })
                    .collect::<Vec<_>>(),
            )
            .into_iter()
            .enumerate()
            {
                context[0] += 0.5 * weight * scaled_states[past][0];
                context[1] += 0.5 * weight * scaled_states[past][1];
            }
        }
        let input_plus_transformed_value: [f64; 2] =
            [states[index][0] + context[0], states[index][1] + context[1]];
        let feed_forward_input: [f64; 2] =
            normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
                &input_plus_transformed_value,
                &learned_coord_scales_after_root_mean_square_normalization,
                1e-6,
            )
            .unwrap();
        [
            input_plus_transformed_value[0]
                + 0.1
                    * calc_gated_layer_output_as_silu_gate_times_up_value(
                        feed_forward_input[0],
                        feed_forward_input[1],
                    ),
            input_plus_transformed_value[1]
                + 0.1
                    * calc_gated_layer_output_as_silu_gate_times_up_value(
                        feed_forward_input[1],
                        feed_forward_input[0],
                    ),
        ]
    })
}
fn main() {
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];

    assert_eq!(
        calc_sequence_block_output_by_normalizing_rotating_and_mixing_past_values_and_gated_features(&[states[0]])[0],
        calc_sequence_block_output_by_normalizing_rotating_and_mixing_past_values_and_gated_features(&states)[0]
    );
}
