//! RMSNorm перед подслоем.

/// RMSNorm без вычитания среднего; gamma задаётся отдельно для каждой координаты.
pub fn root_mean_square_normalization(
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
    use super::root_mean_square_normalization;
    #[test]
    fn shape_and_scale() {
        let output = root_mean_square_normalization(&[3.0, 4.0], &[1.0, 1.0], 1e-8).unwrap();
        assert!(((output[0] * output[0] + output[1] * output[1]) / 2.0 - 1.0).abs() < 1e-7);
        assert!(root_mean_square_normalization(&[1.0], &[], 1e-8).is_err());
    }
}
