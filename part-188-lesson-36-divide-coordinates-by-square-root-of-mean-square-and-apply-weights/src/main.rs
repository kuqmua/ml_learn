// Урок 36.1. Деление координат на корень из среднего квадрата и умножение на веса.
// Связь с принятой терминологией: Нормализация вектора токена по среднеквадратичному значению.
// Зачем здесь эта тема: Следующие архитектурные элементы используют другую нормировку, сохраняющую
//   среднеквадратичный масштаб вектора.
// Почему код устроен так: Делим координаты на RMS с epsilon, не вычитая среднее как в LayerNorm.
// Представь: Для [3, 4] RMS связан со средним чисел 9 и 16; деление уменьшает общий масштаб
//   вектора.
// Нормируем средний квадрат координат и применяем обучаемый масштаб.

fn main() {
    lesson_trace::enable();
    let input_component: [f64; 2] = [3.0, 4.0];
    lesson_trace::trace_step!(input_component);
    let result: Vec<f64> =
        part_188_lesson_36_divide_coordinates_by_square_root_of_mean_square_and_apply_weights::divide_coordinates_by_root_mean_square_then_apply_weights(
            &input_component,
            &[1.0, 1.0],
            // ε=10⁻⁸ защищает от нулевого RMS и почти не меняет обычный ненулевой вектор.
            1e-8,
        )
        .unwrap();
    lesson_trace::trace_step!(result);
    println!("до: {input_component:?}; после RMSNorm: {result:?}");
}
