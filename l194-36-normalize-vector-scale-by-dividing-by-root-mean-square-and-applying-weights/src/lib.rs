//! Урок 194. Нормализация масштаба вектора: деление координат на корень из среднего квадрата и умножение на веса.
//! Связь с принятой терминологией: RMSNorm перед подслоем.

/// RMSNorm без вычитания среднего; gamma задаётся отдельно для каждой координаты.
/// RMSNorm: делим координаты на sqrt(среднее квадратов + epsilon), затем умножаем каждую на её вес gamma.
/// В учебном блоке вход, веса и результат имеют по две координаты.

pub fn normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
    input: &[f64; 2],
    gamma: &[f64; 2],
    epsilon: f64,
) -> Result<[f64; 2], &'static str> {
    if epsilon <= 0.0 {
        return Err("epsilon должен быть положительным");
    }
    let mean_square: f64 = input
        .iter()
        .map(|input_component| input_component * input_component)
        .sum::<f64>()
        / input.len() as f64;
    let scale: f64 = 1.0 / (mean_square + epsilon).sqrt();
    Ok(std::array::from_fn(|index| {
        input[index] * scale * gamma[index]
    }))
}
#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем epsilon и приведение среднего квадрата координат к единице с численным допуском.
    fn rejects_nonpositive_epsilon_and_scales_mean_square_to_one() {
        let output: [f64; 2] = super::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
            &[3.0, 4.0],
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();
        assert!(((output[0] * output[0] + output[1] * output[1]) / 2.0 - 1.0).abs() < 1e-7);
        assert!(super::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(&[3.0, 4.0], &[1.0, 1.0], 0.0).is_err());
    }
}
