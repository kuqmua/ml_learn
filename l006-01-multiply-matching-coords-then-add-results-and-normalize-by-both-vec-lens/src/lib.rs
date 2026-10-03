//! Урок 006. Сравниваем направления двух стрелок независимо от их длин.
//! Сначала умножаем соответствующие числа и складываем результаты, как в уроке 001.
//! Затем делим сумму на длину первой стрелки и на длину второй.
//! Результат 1 означает одно направление, 0 — угол 90°, −1 — противоположные направления.
//! Например, [1, 0] и [10, 0] дают 1: длины разные, но направления совпадают.
//! Для нулевой стрелки направления нет, поэтому функция возвращает ошибку.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;
use l003_01_calc_vec_len_as_square_root_of_sum_of_squared_coords::calc_vec_len_as_square_root_of_sum_of_squared_coords;

pub fn multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
    first_vec: &[f64],
    second_vec: &[f64],
) -> Result<f64, &'static str> {
    let sum_after_multiplying_coords: f64 =
        multiply_matching_coords_then_add_results(first_vec, second_vec)?;
    let multiplied_vec_lens: f64 = calc_vec_len_as_square_root_of_sum_of_squared_coords(first_vec)
        * calc_vec_len_as_square_root_of_sum_of_squared_coords(second_vec);
    if multiplied_vec_lens == 0.0 {
        return Err("у нулевого вектора нет направления");
    }
    Ok(sum_after_multiplying_coords / multiplied_vec_lens)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `reuses_earlier_lessons_and_rejects_zero_vec` для этого примера.
    fn reuses_earlier_lessons_and_rejects_zero_vec() {
        assert_eq!(
            super::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
                &[1.0, 0.0],
                &[0.0, 1.0]
            ),
            Ok(0.0)
        );
        assert_eq!(
            super::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
                &[1.0, 0.0],
                &[-1.0, 0.0]
            ),
            Ok(-1.0)
        );
        assert!(
            super::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
                &[1.0, 0.0],
                &[0.0, 0.0]
            )
            .is_err()
        );
    }
}
