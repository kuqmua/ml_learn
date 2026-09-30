//! Урок 004. Квадрат расстояния: сложение квадратов разностей координат.

/// Сумма квадратов покоординатных разностей.
/// Квадрат евклидова расстояния: вычитаем соответствующие координаты, возводим разности в квадрат и складываем.
/// Оба массива имеют одинаковую длину `N`; число координат проверяет компилятор.
/// Пустые точки и неконечные координаты отклоняются во время выполнения.

pub fn calculate_squared_point_distance_by_summing_squared_coordinate_differences<
    const N: usize,
>(
    left: &[f64; N],
    right: &[f64; N],
) -> Result<f64, &'static str> {
    if N == 0 || left.iter().chain(right).any(|value| !value.is_finite()) {
        return Err("точки должны содержать конечные координаты");
    }
    let mut squared_sum: f64 = 0.0;
    for index in 0..left.len() {
        let difference: f64 = left[index] - right[index];
        squared_sum += difference * difference;
    }
    if !squared_sum.is_finite() {
        return Err("квадрат расстояния выходит за пределы f64");
    }
    Ok(squared_sum)
}
