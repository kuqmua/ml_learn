//! Урок 004. Измеряем, насколько далеко друг от друга две точки, пока без извлечения корня.
//! Вычитаем числа на одинаковых местах, умножаем каждую разницу саму на себя и складываем.
//! Для [1, 2] и [4, 6] получаем 3×3 + 4×4 = 25.
//! Ноль означает одну и ту же точку. Чем больше результат, тем дальше точки друг от друга.
//! Чтобы получить обычное расстояние, из этого результата нужно взять квадратный корень.

pub fn calc_squared_point_dist_by_summing_squared_coord_diffs<const N: usize>(
    point1: &[f64; N],
    point2: &[f64; N],
) -> Result<f64, &'static str> {
    if N == 0 || point1.iter().chain(point2).any(|value| !value.is_finite()) {
        return Err("точки должны быть непустыми и содержать только конечные координаты");
    }
    let mut squared_sum: f64 = 0.0;
    for index in 0..point1.len() {
        let diff: f64 = point1[index] - point2[index];
        squared_sum += diff * diff;
    }
    if !squared_sum.is_finite() {
        return Err("квадрат расстояния выходит за пределы f64");
    }
    Ok(squared_sum)
}
