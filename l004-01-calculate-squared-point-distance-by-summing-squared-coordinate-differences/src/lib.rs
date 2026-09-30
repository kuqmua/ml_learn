//! Урок 004. Квадрат расстояния: сложение квадратов разностей координат.

/// Сумма квадратов покоординатных разностей.
/// Квадрат евклидова расстояния: вычитаем соответствующие координаты, возводим разности в квадрат и складываем.
use lesson_trace::{trace_note, trace_step};

pub fn calculate_squared_point_distance_by_summing_squared_coordinate_differences(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if left.len() != right.len() {
        trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("точки должны иметь одинаковое число координат");
    }
    trace_note!("Сохраняем результат этого шага в `squared_sum`.");
    let mut squared_sum: f64 = 0.0;
    trace_step!(squared_sum);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for index in 0..left.len() {
        trace_step!(index);
        trace_note!("Сохраняем результат этого шага в `difference`.");
        let difference: f64 = left[index] - right[index];
        trace_step!(difference);
        trace_note!("Обновляем значение результатом текущего вычисления.");
        squared_sum += difference * difference;
        trace_step!(squared_sum);
    }
    trace_note!("Возвращаем успешный результат.");
    Ok(squared_sum)
}
