// Урок 36.1. Нормализация вектора токена по среднеквадратичному значению.
// Нормируем средний квадрат координат и применяем обучаемый масштаб.

fn main() {
    lesson_trace::enable();
    let input_component: [f64; 2] = [3.0, 4.0];
    lesson_trace::trace_step!(input_component);
    let result: Vec<f64> =
        part_188_lesson_36_normalize_token_vector_by_root_mean_square::normalize_vector_by_root_mean_square(
            &input_component,
            &[1.0, 1.0],
            // ε=10⁻⁸ защищает от нулевого RMS и почти не меняет обычный ненулевой вектор.
            1e-8,
        )
        .unwrap();
    lesson_trace::trace_step!(result);
    println!("до: {input_component:?}; после RMSNorm: {result:?}");
}
