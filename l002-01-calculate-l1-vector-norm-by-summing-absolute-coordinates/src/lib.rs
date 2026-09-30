//! Урок 002. Длина пути вдоль осей (норма L1): сложение модулей координат вектора.

/// Получаем норму L1: складываем модули координат — длины перемещений вдоль каждой оси.
/// Для [3, 4] это 7; обычная длина прямого отрезка (норма L2) равна 5.
pub fn calculate_l1_vector_norm_by_summing_absolute_coordinates(vector: &[f64]) -> f64 {
    lesson_trace::trace_note!("Сохраняем результат этого шага в `sum`.");
    let mut sum: f64 = 0.0;
    lesson_trace::trace_step!(sum);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for &coordinate in vector {
        lesson_trace::trace_step!(coordinate);
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        sum += if coordinate < 0.0 {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            -coordinate
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            coordinate
        };
        lesson_trace::trace_step!(sum);
    }
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    sum
}
