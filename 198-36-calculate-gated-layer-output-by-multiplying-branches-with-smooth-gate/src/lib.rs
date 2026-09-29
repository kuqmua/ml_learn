//! Урок 198. Выход управляемого слоя: умножение двух ветвей с плавным управлением вкладом.
//! Связь с принятой терминологией: SwiGLU в feed-forward слое.

/// SiLU(gate) * up — промежуточный выход gated FFN.
/// SwiGLU: gate·up / (1 + e^(−gate)); значение одной ветви управляет вкладом другой.
pub fn calculate_gated_layer_output_as_gate_times_up_value_over_one_plus_e_to_negative_gate(
    gate: f64,
    up_projection: f64,
) -> f64 {
    gate / (1.0 + (-gate).exp()) * up_projection
}
