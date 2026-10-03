//! Урок 194. Нормализация масштаба вектора: деление координат на корень из среднего квадрата и умножение на веса.
//! Связь с принятой терминологией: RMSNorm перед подслоем.

/// RMSNorm без вычитания среднего; coordinate_weights задаётся отдельно для каждой координаты.
/// RMSNorm: делим координаты на sqrt(среднее квадратов + epsilon), затем умножаем каждую на её вес coordinate_weights.
/// В учебном блоке вход, веса и результат имеют по две координаты.

pub fn normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights_to_control_scale_without_centering(
    input_vector: &[f64; 2],
    coordinate_weights: &[f64; 2],
    positive_stabilizer_preventing_division_by_zero_for_zero_vector: f64,
) -> Result<[f64; 2], &'static str> {
    if positive_stabilizer_preventing_division_by_zero_for_zero_vector <= 0.0 {
        return Err(
            "positive_stabilizer_preventing_division_by_zero_for_zero_vector должен быть положительным",
        );
    }
    let mean_square: f64 = input_vector
        .iter()
        .map(|input_component| input_component * input_component)
        .sum::<f64>()
        / input_vector.len() as f64;
    let inverse_stabilized_root_mean_square_for_rescaling_coordinates: f64 = 1.0
        / (mean_square + positive_stabilizer_preventing_division_by_zero_for_zero_vector).sqrt();
    Ok(std::array::from_fn(|index| {
        input_vector[index]
            * inverse_stabilized_root_mean_square_for_rescaling_coordinates
            * coordinate_weights[index]
    }))
}
#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем epsilon и приведение среднего квадрата координат к единице с численным допуском.
    fn rejects_nonpositive_epsilon_and_scales_mean_square_to_one() {
        let output: [f64; 2] = super::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights_to_control_scale_without_centering(
            &[3.0, 4.0],
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();
        assert!(((output[0] * output[0] + output[1] * output[1]) / 2.0 - 1.0).abs() < 1e-7);
        assert!(super::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights_to_control_scale_without_centering(&[3.0, 4.0], &[1.0, 1.0], 0.0).is_err());
    }
}
