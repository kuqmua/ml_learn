//! Урок 189. Поворот пар координат запроса и ключа в зависимости от позиции в тексте.
//! Связь с принятой терминологией: Вращательные позиционные признаки RoPE.

/// Вращаем пару координат согласно позиции; для многомерной головы это делают по парам.
pub fn rotate_vector_coordinate_pair_by_token_position(
    vector: [f64; 2],
    position: usize,
    theta: f64,
) -> [f64; 2] {
    let angle: f64 = position as f64 * theta;
    lesson_trace::trace_step!(angle);
    let (sine_value, cosine_value): (f64, f64) = angle.sin_cos();
    lesson_trace::trace_step!(sine_value);
    lesson_trace::trace_step!(cosine_value);
    [
        vector[0] * cosine_value - vector[1] * sine_value,
        vector[0] * sine_value + vector[1] * cosine_value,
    ]
}
#[cfg(test)]
mod tests {
    #[test]
    /// Поворот сохраняет сумму квадратов координат, а значит и длину вектора.
    fn rotation_preserves_sum_of_squared_coordinates() {
        let input_value: [f64; 2] = [3.0, 4.0];
        let second_input_value: [f64; 2] =
            super::rotate_vector_coordinate_pair_by_token_position(input_value, 7, 0.1);
        assert!(
            (second_input_value[0] * second_input_value[0]
                + second_input_value[1] * second_input_value[1]
                - 25.0)
                .abs()
                < 1e-10
        );
    }
}
