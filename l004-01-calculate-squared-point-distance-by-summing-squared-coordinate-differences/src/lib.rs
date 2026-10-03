//! Урок 004. Квадрат расстояния: сложение квадратов разностей координат.

/// Сумма квадратов покоординатных разностей.
/// Квадрат евклидова расстояния: вычитаем соответствующие координаты, возводим разности в квадрат и складываем.
/// Оба массива имеют одинаковую длину `N`; число координат проверяет компилятор.
/// Пустые точки и неконечные координаты отклоняются во время выполнения.

pub fn calculate_squared_point_distance_by_summing_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther<
    const N: usize,
>(
    first_point: &[f64; N],
    second_point: &[f64; N],
) -> Result<f64, &'static str> {
    if N == 0
        || first_point
            .iter()
            .chain(second_point)
            .any(|value| !value.is_finite())
    {
        return Err("точки должны быть непустыми и содержать только конечные координаты");
    }
    let mut squared_sum: f64 = 0.0;
    for index in 0..first_point.len() {
        let difference: f64 = first_point[index] - second_point[index];
        squared_sum += difference * difference;
    }
    if !squared_sum.is_finite() {
        return Err("квадрат расстояния выходит за пределы f64");
    }
    Ok(squared_sum)
}
