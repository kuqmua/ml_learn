//! Вычисления и примеры урока part-001-lesson-01-multiply-coordinates-and-add.

/// Умножаем соответствующие координаты и складываем результаты.
pub fn multiply_matching_coordinates_then_add(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("векторы должны быть одинаковой длины");
    }
    let mut sum = 0.0;
    for index in 0..left.len() {
        sum += left[index] * right[index];
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    #[test]
    fn handles_perpendicular_and_mismatched_vectors() {
        assert_eq!(
            super::multiply_matching_coordinates_then_add(&[1.0, 2.0], &[-2.0, 1.0]),
            Ok(0.0)
        );
        assert!(super::multiply_matching_coordinates_then_add(&[1.0], &[1.0, 2.0]).is_err());
    }
}
