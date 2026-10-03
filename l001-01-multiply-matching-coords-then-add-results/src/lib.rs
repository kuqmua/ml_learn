//! Урок 001. Умножаем числа на одинаковых местах в двух списках и складываем результаты.
//! Например, для [1, 2] и [3, 4] получаем 1×3 + 2×4 = 11.
//! Так можно посчитать сумму признаков, каждый из которых умножен на свой вес.
//! Если представить списки как стрелки, знак результата говорит об угле между ними:
//! плюс — угол меньше 90°, минус — больше 90°, ноль — угол 90° или нулевая стрелка.
//! Величина результата зависит и от направлений, и от длин стрелок.
//! Списки должны содержать одинаковое количество чисел.

pub fn multiply_matching_coords_then_add_results(
    first_vec: &[f64],
    second_vec: &[f64],
) -> Result<f64, &'static str> {
    if first_vec.len() != second_vec.len() {
        return Err("векторы должны иметь одинаковое число координат");
    }
    let mut sum_after_multiplying_matching_coords: f64 = 0.0;
    for index in 0..first_vec.len() {
        sum_after_multiplying_matching_coords += first_vec[index] * second_vec[index];
    }
    Ok(sum_after_multiplying_matching_coords)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `handles_perpendicular_and_mismatched_vecs` для этого примера.
    fn handles_perpendicular_and_mismatched_vecs() {
        assert_eq!(
            super::multiply_matching_coords_then_add_results(&[1.0, 2.0], &[-2.0, 1.0]),
            Ok(0.0)
        );
        assert!(super::multiply_matching_coords_then_add_results(&[1.0], &[1.0, 2.0]).is_err());
    }
}
