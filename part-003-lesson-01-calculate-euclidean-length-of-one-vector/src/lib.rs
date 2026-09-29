//! Вычисления и примеры урока part-003-lesson-01-calculate-euclidean-length-of-one-vector.

/// Корень методом Ньютона для небольших учебных входов.
fn square_root_with_newton_method(value: f64) -> f64 {
    // Проверяем ожидаемое свойство учебного примера.
    assert!(value >= 0.0, "корень из отрицательного числа");
    // Выбираем дальнейший шаг по выполнению условия.
    if value == 0.0 {
        // Завершаем вычисление с полученным результатом.
        return 0.0;
    }
    // Начинаем с положительной оценки: value при value > 1, иначе 1, чтобы не делить на ноль.
    let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
    // 80 шагов — консервативный предел для небольших учебных входов, не часть формулы корня.
    // В общем случае число шагов лучше определять по изменению оценки или требуемой точности.
    for _ in 0..80 {
        // Метод Ньютона для f(x)=x²−value: x−f(x)/f'(x) = (x+value/x)/2.
        estimate = (estimate + value / estimate) / 2.0;
    }
    // Используем подготовленное значение в следующем шаге примера.
    estimate
}

/// Длина вектора использует вычисление из урока 01.1.
pub fn euclidean_norm_of_vector(vector: &[f64]) -> f64 {
    // Сохраняем результат этого шага в `squared_length`.
    let squared_length: f64 =
        part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::multiply_matching_coordinates_of_two_vectors_then_add(
            vector, vector,
        )
        // Используем результат, ожидая успешного выполнения шага.
        .expect("длина вектора сравнивает его с самим собой");
    // Используем подготовленное значение в следующем шаге примера.
    square_root_with_newton_method(squared_length)
}
