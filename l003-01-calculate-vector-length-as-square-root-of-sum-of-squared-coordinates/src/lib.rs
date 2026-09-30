//! Урок 003. Длина вектора: квадратный корень из суммы квадратов координат.

/// Приближаем квадратный корень, многократно усредняя оценку и число, делённое на оценку.
/// Это метод Ньютона для небольших учебных входов.
/// Учебный аналог `f64::sqrt`: показывает шаги метода Ньютона и может работать медленнее.
/// Для отрицательного входа здесь panic, тогда как `sqrt` возвращает NaN.
/// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;

use lesson_trace::{trace_note, trace_step};

fn approximate_square_root_by_repeated_averaging(value: f64) -> f64 {
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(value >= 0.0, "корень из отрицательного числа");
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if value == 0.0 {
        trace_note!("Завершаем вычисление с полученным результатом.");
        return 0.0;
    }
    trace_note!(
        "Начинаем с положительной оценки: value при value > 1, иначе 1, чтобы не делить на ноль."
    );
    let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
    trace_step!(estimate);
    trace_note!(
        "80 шагов — консервативный предел для небольших учебных входов, не часть формулы корня."
    );
    trace_note!(
        "В общем случае число шагов лучше определять по изменению оценки или требуемой точности."
    );
    for _ in 0..80 {
        trace_note!("Метод Ньютона для f(x)=x²−value: x−f(x)/f'(x) = (x+value/x)/2.");
        estimate = (estimate + value / estimate) / 2.0;
        trace_step!(estimate);
    }
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    estimate
}

/// Берём квадратный корень из суммы квадратов координат: sqrt(v₁² + v₂² + …).
/// Получаем длину вектора — в математике это евклидова норма (норма L2).
/// По теореме Пифагора это расстояние от начала координат до конца вектора.
/// Например, для [3, 4]: sqrt(9 + 16) = 5.
pub fn calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(vector: &[f64]) -> f64 {
    trace_note!("Умножаем каждую координату на саму себя и складываем: получаем сумму квадратов.");
    trace_note!("Передаём один вектор дважды, поэтому число координат гарантированно совпадает.");
    let sum_of_squared_coordinates: f64 =
        calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(vector, vector)
            .expect("у вектора и его самого одинаковое число координат");
    trace_step!(sum_of_squared_coordinates);
    trace_note!("Извлекаем корень из суммы квадратов и получаем длину вектора.");
    approximate_square_root_by_repeated_averaging(sum_of_squared_coordinates)
}
