//! Урок 198. Выход управляемого слоя: умножение двух ветвей с плавным управлением вкладом.
//! Связь с принятой терминологией: SwiGLU в feed-forward слое.

/// SiLU(gate) * up — промежуточный выход gated FFN.
/// SwiGLU: gate·up / (1 + e^(−gate)); значение одной ветви управляет вкладом другой.
pub fn calculate_gated_layer_output_as_silu_gate_times_up_value_where_0_gate_blocks_and_gate_multiplier_can_be_negative_or_greater_than_1(
    gate: f64,
    up_projection: f64,
) -> f64 {
    let sigmoid_denominator_controlling_gate_suppression: f64 = 1.0 + (-gate).exp();
    let silu_gate_value_that_can_be_negative_and_is_not_bounded_by_1: f64 =
        gate / sigmoid_denominator_controlling_gate_suppression;
    silu_gate_value_that_can_be_negative_and_is_not_bounded_by_1 * up_projection
}
