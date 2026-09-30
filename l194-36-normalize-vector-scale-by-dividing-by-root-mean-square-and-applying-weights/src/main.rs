// Урок 36.1. Нормализация масштаба вектора: деление координат на корень из среднего квадрата и умножение на веса.
// Связь с принятой терминологией: Нормализация вектора токена по среднеквадратичному значению.
// Зачем здесь эта тема: Следующие архитектурные элементы используют другую нормировку, сохраняющую
//   среднеквадратичный масштаб вектора.
// Почему код устроен так: Делим координаты на RMS с epsilon, не вычитая среднее как в LayerNorm.
// Представь: Для [3, 4] RMS связан со средним чисел 9 и 16; деление уменьшает общий масштаб
//   вектора.
// Нормируем средний квадрат координат и применяем обучаемый масштаб.

use l194_36_normalize_vector_scale_by_dividing_by_root_mean_square_and_applying_weights::normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights;

fn main() {
    let input_component: [f64; 2] = [3.0, 4.0];
    let _result: [f64; 2] =
        normalize_vector_scale_by_dividing_coordinates_by_root_mean_square_then_applying_weights(
            &input_component,
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();
}
