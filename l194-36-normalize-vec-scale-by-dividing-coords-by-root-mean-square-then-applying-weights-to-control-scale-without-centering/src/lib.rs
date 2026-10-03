//! Урок 194. Нормализация масштаба вектора: деление координат на корень из среднего квадрата и умножение на веса.
//! Связь с принятой терминологией: RMSNorm перед подслоем.

/// RMSNorm без вычитания среднего; coord_weights задаётся отдельно для каждой координаты.
/// RMSNorm: делим координаты на sqrt(среднее квадратов + epsilon), затем умножаем каждую на её вес coord_weights.
/// В учебном блоке вход, веса и результат имеют по две координаты.

pub fn normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights_to_control_scale_without_centering(
    input_vec: &[f64; 2],
    coord_weights: &[f64; 2],
    pos_stabilizer_preventing_division_by_zero_for_zero_vec: f64,
) -> Result<[f64; 2], &'static str> {
    if pos_stabilizer_preventing_division_by_zero_for_zero_vec <= 0.0 {
        return Err(
            "pos_stabilizer_preventing_division_by_zero_for_zero_vec должен быть положительным",
        );
    }
    let mean_square: f64 = input_vec
        .iter()
        .map(|input_component| input_component * input_component)
        .sum::<f64>()
        / input_vec.len() as f64;
    let inverse_stabilized_root_mean_square_for_rescaling_coords: f64 =
        1.0 / (mean_square + pos_stabilizer_preventing_division_by_zero_for_zero_vec).sqrt();
    Ok(std::array::from_fn(|index| {
        input_vec[index]
            * inverse_stabilized_root_mean_square_for_rescaling_coords
            * coord_weights[index]
    }))
}
#[cfg(test)]
mod tests {
    use lesson_float_comparison::check_f64_eq_1e_minus_7;

    #[test]
    /// Проверяем epsilon и приведение среднего квадрата координат к единице с численным допуском.
    fn rejects_nonpos_epsilon_and_scales_mean_square_to_one() {
        let output: [f64; 2] = super::normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights_to_control_scale_without_centering(
            &[3.0, 4.0],
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();
        assert!(check_f64_eq_1e_minus_7(
            (output[0] * output[0] + output[1] * output[1]) / 2.0,
            1.0
        ));
        assert!(super::normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights_to_control_scale_without_centering(&[3.0, 4.0], &[1.0, 1.0], 0.0).is_err());
    }
}
