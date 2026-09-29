//! Вычисления и примеры урока part-002-lesson-01-sum-absolute-values-of-vector-coordinates.

/// Складываем модули координат.
pub fn sum_absolute_values_of_vector_coordinates(vector: &[f64]) -> f64 {
    // Сохраняем результат этого шага в `sum`.
    let mut sum: f64 = 0.0;
    lesson_trace::trace_step!(sum);
    // Повторяем расчёт для каждого элемента последовательности.
    for &coordinate in vector {
        lesson_trace::trace_step!(coordinate);
        // Обновляем значение результатом текущего вычисления.
        sum += if coordinate < 0.0 {
            // Используем подготовленное значение в следующем шаге примера.
            -coordinate
        // Обрабатываем случай, когда предыдущее условие не выполнено.
        } else {
            // Используем подготовленное значение в следующем шаге примера.
            coordinate
        };
        lesson_trace::trace_step!(sum);
    }
    // Используем подготовленное значение в следующем шаге примера.
    sum
}
