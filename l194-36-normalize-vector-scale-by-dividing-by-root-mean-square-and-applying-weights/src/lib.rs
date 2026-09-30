//! Урок 194. Нормализация масштаба вектора: деление координат на корень из среднего квадрата и умножение на веса.
//! Связь с принятой терминологией: RMSNorm перед подслоем.

/// RMSNorm без вычитания среднего; gamma задаётся отдельно для каждой координаты.
/// RMSNorm: делим координаты на sqrt(среднее квадратов + epsilon), затем умножаем каждую на её вес gamma.
/// Массивы входа, весов и результата имеют одну и ту же длину `N`.

pub fn normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights<
    const N: usize,
>(
    input: &[f64; N],
    gamma: &[f64; N],
    epsilon: f64,
) -> Result<[f64; N], &'static str> {
    if N == 0 || epsilon <= 0.0 {
        return Err("неверная форма или epsilon");
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
    /// Проверяем размеры весов и приведение среднего квадрата координат к единице с численным допуском.
    fn rejects_mismatched_weights_and_scales_mean_square_to_one() {
        let output: [f64; 2] = super::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
            &[3.0, 4.0],
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();
        assert!(((output[0] * output[0] + output[1] * output[1]) / 2.0 - 1.0).abs() < 1e-7);
        assert!(super::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(&[], &[], 1e-8).is_err());
    }
}
