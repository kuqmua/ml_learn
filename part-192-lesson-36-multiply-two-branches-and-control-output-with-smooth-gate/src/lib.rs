//! Урок 192. Умножение двух ветвей с плавным управлением вкладом одной из них.
//! Связь с принятой терминологией: SwiGLU в feed-forward слое.

/// SiLU(gate) * up — промежуточный выход gated FFN.
/// SwiGLU: gate·up / (1 + e^(−gate)); значение одной ветви управляет вкладом другой.
pub fn multiply_gate_and_up_value_then_divide_by_one_plus_e_to_negative_gate(
    gate: f64,
    up_projection: f64,
) -> f64 {
    gate / (1.0 + (-gate).exp()) * up_projection
}
