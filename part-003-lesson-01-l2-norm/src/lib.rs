//! Вычисления и примеры урока part-003-lesson-01-l2-norm.

/// Корень методом Ньютона для небольших учебных входов.
fn square_root_with_newton_method(value: f64) -> f64 {
    assert!(value >= 0.0, "корень из отрицательного числа");
    if value == 0.0 {
        return 0.0;
    }
    let mut estimate = if value > 1.0 { value } else { 1.0 };
    for _ in 0..80 {
        estimate = (estimate + value / estimate) / 2.0;
    }
    estimate
}

/// Длина вектора использует вычисление из урока 01.1.
pub fn l2_norm(vector: &[f64]) -> f64 {
    let squared_length = lesson_001::multiply_matching_coordinates_then_add(vector, vector)
        .expect("длина вектора сравнивает его с самим собой");
    square_root_with_newton_method(squared_length)
}
