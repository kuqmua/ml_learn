//! Вычисления и примеры урока part-005-lesson-01-cosine-similarity.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
pub fn cosine_similarity(left: &[f64], right: &[f64]) -> Result<f64, &'static str> {
    let numerator = lesson_001::multiply_matching_coordinates_then_add(left, right)?;
    let denominator = lesson_003::l2_norm(left) * lesson_003::l2_norm(right);
    if denominator == 0.0 {
        return Err("у нулевого вектора нет направления");
    }
    Ok(numerator / denominator)
}

#[cfg(test)]
mod tests {
    #[test]
    fn reuses_earlier_lessons_and_rejects_zero_vector() {
        assert_eq!(super::cosine_similarity(&[1.0, 0.0], &[0.0, 1.0]), Ok(0.0));
        assert_eq!(
            super::cosine_similarity(&[1.0, 0.0], &[-1.0, 0.0]),
            Ok(-1.0)
        );
        assert!(super::cosine_similarity(&[1.0, 0.0], &[0.0, 0.0]).is_err());
    }
}
