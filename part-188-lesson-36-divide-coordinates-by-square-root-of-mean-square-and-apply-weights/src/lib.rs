//! Урок 188. Деление координат на корень из среднего квадрата и умножение на веса.
//! Связь с принятой терминологией: RMSNorm перед подслоем.

/// RMSNorm без вычитания среднего; gamma задаётся отдельно для каждой координаты.
/// RMSNorm: делим координаты на sqrt(среднее квадратов + epsilon), затем умножаем каждую на её вес gamma.
pub fn divide_coordinates_by_root_mean_square_then_apply_weights(
    input: &[f64],
    gamma: &[f64],
    epsilon: f64,
) -> Result<Vec<f64>, &'static str> {
    if input.is_empty() || input.len() != gamma.len() || epsilon <= 0.0 {
        return Err("неверная форма или epsilon");
    }
    let mean_square: f64 = input
        .iter()
        .map(|input_component| input_component * input_component)
        .sum::<f64>()
        / input.len() as f64;
    lesson_trace::trace_step!(mean_square);
    // epsilon добавляем до корня, чтобы RMS не оказался нулём для нулевого вектора.
    let scale: f64 = 1.0 / (mean_square + epsilon).sqrt();
    lesson_trace::trace_step!(scale);
    Ok(input
        .iter()
        .zip(gamma)
        .map(|(&input_component, &gamma_value)| input_component * scale * gamma_value)
        .collect())
}
#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем размеры весов и приведение среднего квадрата координат к единице с численным допуском.
    fn rejects_mismatched_weights_and_scales_mean_square_to_one() {
        let output: Vec<f64> = super::divide_coordinates_by_root_mean_square_then_apply_weights(
            &[3.0, 4.0],
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();
        assert!(((output[0] * output[0] + output[1] * output[1]) / 2.0 - 1.0).abs() < 1e-7);
        assert!(
            super::divide_coordinates_by_root_mean_square_then_apply_weights(&[1.0], &[], 1e-8)
                .is_err()
        );
    }
}
