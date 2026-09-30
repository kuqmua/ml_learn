//! Урок 002. Длина пути вдоль осей (норма L1): сложение модулей координат вектора.

/// Получаем норму L1: складываем модули координат — длины перемещений вдоль каждой оси.
/// Для [3, 4] это 7; обычная длина прямого отрезка (норма L2) равна 5.

pub fn calculate_l1_vector_norm_by_summing_absolute_coordinates(vector: &[f64]) -> f64 {
    let mut sum: f64 = 0.0;
    for &coordinate in vector {
        sum += if coordinate < 0.0 {
            -coordinate
        } else {
            coordinate
        };
    }
    sum
}
