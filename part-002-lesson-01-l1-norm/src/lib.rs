//! Вычисления и примеры урока part-002-lesson-01-l1-norm.

/// Складываем модули координат.
pub fn l1_norm(vector: &[f64]) -> f64 {
    let mut sum = 0.0;
    for &coordinate in vector {
        sum += if coordinate < 0.0 {
            -coordinate
        } else {
            coordinate
        };
    }
    sum
}
