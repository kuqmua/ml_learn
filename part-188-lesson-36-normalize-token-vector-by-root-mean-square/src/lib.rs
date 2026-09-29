//! RMSNorm перед подслоем.

/// RMSNorm без вычитания среднего; gamma задаётся отдельно для каждой координаты.
pub fn normalize_vector_by_root_mean_square(
    input: &[f64],
    gamma: &[f64],
    epsilon: f64,
) -> Result<Vec<f64>, &'static str> {
    if input.is_empty() || input.len() != gamma.len() || epsilon <= 0.0 {
        return Err("неверная форма или epsilon");
    }
    let mean_square = input
        .iter()
        .map(|input_component| input_component * input_component)
        .sum::<f64>()
        / input.len() as f64;
    let scale = 1.0 / (mean_square + epsilon).sqrt();
    Ok(input
        .iter()
        .zip(gamma)
        .map(|(&input_component, &gamma_value)| input_component * scale * gamma_value)
        .collect())
}
#[cfg(test)]
mod tests {
    #[test]
    fn shape_and_scale() {
        let output =
            super::normalize_vector_by_root_mean_square(&[3.0, 4.0], &[1.0, 1.0], 1e-8).unwrap();
        assert!(((output[0] * output[0] + output[1] * output[1]) / 2.0 - 1.0).abs() < 1e-7);
        assert!(super::normalize_vector_by_root_mean_square(&[1.0], &[], 1e-8).is_err());
    }
}
