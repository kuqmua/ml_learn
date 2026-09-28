//! Вычисления и примеры урока part-005-lesson-01-cosine-similarity.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
pub fn cosine_similarity(left: &[f64], right: &[f64]) -> Result<f64, &'static str> {
    // Сохраняем результат этого шага в `numerator`.
    let numerator = lesson_001::multiply_matching_coordinates_then_add(left, right)?;
    // Сохраняем результат этого шага в `denominator`.
    let denominator = lesson_003::l2_norm(left) * lesson_003::l2_norm(right);
    // Выбираем дальнейший шаг по выполнению условия.
    if denominator == 0.0 {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("у нулевого вектора нет направления");
    }
    // Возвращаем успешный результат.
    Ok(numerator / denominator)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {
    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `reuses_earlier_lessons_and_rejects_zero_vector` для этого примера.
    fn reuses_earlier_lessons_and_rejects_zero_vector() {
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(super::cosine_similarity(&[1.0, 0.0], &[0.0, 1.0]), Ok(0.0));
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(
            // Задаём именованное поле или параметр.
            super::cosine_similarity(&[1.0, 0.0], &[-1.0, 0.0]),
            // Возвращаем успешный результат.
            Ok(-1.0)
        );
        // Проверяем ожидаемое свойство учебного примера.
        assert!(super::cosine_similarity(&[1.0, 0.0], &[0.0, 0.0]).is_err());
    }
}
