//! Урок 004. Квадрат расстояния: сложение квадратов разностей координат.

/// Сумма квадратов покоординатных разностей.
/// Квадрат евклидова расстояния: вычитаем соответствующие координаты, возводим разности в квадрат и складываем.

pub fn calculate_squared_point_distance_by_summing_squared_coordinate_differences(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("точки должны иметь одинаковое число координат");
    }
    let mut squared_sum: f64 = 0.0;
    for index in 0..left.len() {
        let difference: f64 = left[index] - right[index];
        squared_sum += difference * difference;
    }
    Ok(squared_sum)
}
