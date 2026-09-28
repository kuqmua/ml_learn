//! SwiGLU в feed-forward слое.

/// SiLU(gate) * up — промежуточный выход gated FFN.
pub fn swiglu(gate: f64, up_projection: f64) -> f64 {
    gate / (1.0 + (-gate).exp()) * up_projection
}
