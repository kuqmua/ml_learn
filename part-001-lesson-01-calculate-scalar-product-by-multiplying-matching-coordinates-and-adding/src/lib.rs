//! Урок 001. Скалярное произведение: умножение соответствующих координат двух векторов и сложение произведений.

/// Умножаем соответствующие координаты и складываем результаты.
/// Скалярное произведение: умножаем соответствующие координаты двух векторов и складываем произведения.
pub fn calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
    // Задаём именованное поле или параметр.
    left: &[f64],
    // Задаём именованное поле или параметр.
    right: &[f64],
    // Указываем тип возвращаемого значения.
) -> Result<f64, &'static str> {
    // Выбираем дальнейший шаг по выполнению условия.
    if left.len() != right.len() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("векторы должны быть одинаковой длины");
    }
    // Сохраняем результат этого шага в `sum`.
    let mut sum: f64 = 0.0;
    lesson_trace::trace_step!(sum);
    // Повторяем расчёт для каждого элемента последовательности.
    for index in 0..left.len() {
        lesson_trace::trace_step!(index);
        // Обновляем значение результатом текущего вычисления.
        sum += left[index] * right[index];
        lesson_trace::trace_step!(sum);
    }
    // Возвращаем успешный результат.
    Ok(sum)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {
    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `handles_perpendicular_and_mismatched_vectors` для этого примера.
    fn handles_perpendicular_and_mismatched_vectors() {
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(
            // Задаём именованное поле или параметр.
            super::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                &[1.0, 2.0],
                &[-2.0, 1.0]
            ),
            // Возвращаем успешный результат.
            Ok(0.0)
        );
        // Проверяем ожидаемое свойство учебного примера.
        assert!(
            super::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                &[1.0],
                &[1.0, 2.0]
            )
            .is_err()
        );
    }
}
