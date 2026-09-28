//! Вычисления и примеры урока part-004-lesson-01-distance.

/// Сумма квадратов покоординатных разностей.
pub fn squared_distance(left: &[f64], right: &[f64]) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("точки должны иметь одинаковое число координат");
    }
    let mut squared_sum = 0.0;
    for index in 0..left.len() {
        let difference = left[index] - right[index];
        squared_sum += difference * difference;
    }
    Ok(squared_sum)
}

/// Расстояние — длина вектора разностей; используем норму из урока 01.3.
pub fn distance(left: &[f64], right: &[f64]) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("точки должны иметь одинаковое число координат");
    }
    let differences: Vec<f64> = left.iter().zip(right).map(|(&a, &b)| a - b).collect();
    Ok(lesson_003::l2_norm(&differences))
}
