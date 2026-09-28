// Урок 36.1. RMSNorm перед подслоем.
// Нормируем средний квадрат координат и применяем обучаемый масштаб.

use part_188_lesson_36_rmsnorm::rms_norm;
fn main() {
    let x = [3.0, 4.0];
    let result = rms_norm(&x, &[1.0, 1.0], 1e-8).unwrap();
    println!("до: {x:?}; после RMSNorm: {result:?}");
}
