//! Урок 198. Выход управляемого слоя: умножение двух ветвей с плавным управлением вкладом.

/// SiLU(gate) * up — промежуточный выход gated FFN.
/// SwiGLU: gate·up / (1 + e^(−gate)); значение одной ветви управляет вкладом другой.
pub fn calc_gated_layer_output_as_silu_gate_times_up_value(gate: f64, up_projection: f64) -> f64 {
    let sigmoid_denominator_controlling_gate_suppression: f64 = 1.0 + (-gate).exp();
    let silu_gate_value: f64 = gate / sigmoid_denominator_controlling_gate_suppression;
    silu_gate_value * up_projection
}
