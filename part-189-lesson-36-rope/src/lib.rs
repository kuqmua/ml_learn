//! Вращательные позиционные признаки RoPE.

/// Вращаем пару координат согласно позиции; для многомерной головы это делают по парам.
pub fn rotate_pair(vector: [f64; 2], position: usize, theta: f64) -> [f64; 2] {
    let angle = position as f64 * theta;
    let (sin, cos) = angle.sin_cos();
    [
        vector[0] * cos - vector[1] * sin,
        vector[0] * sin + vector[1] * cos,
    ]
}
#[cfg(test)]
mod tests {
    use super::rotate_pair;
    #[test]
    fn rotation_preserves_norm() {
        let input_value = [3.0, 4.0];
        let second_input_value = rotate_pair(input_value, 7, 0.1);
        assert!(
            (second_input_value[0] * second_input_value[0]
                + second_input_value[1] * second_input_value[1]
                - 25.0)
                .abs()
                < 1e-10
        );
    }
}
