// Урок 41.4. QK-Norm перед attention.
// Нормируем Q и K отдельно до сравнения, затем применяем позиционное вращение.

use part_208_lesson_41_rmsnorm::rms_norm;
use part_209_lesson_41_rope::rotate_pair;
fn main() {
    let q = rms_norm(&[2.0, 1.0], &[1.0, 1.0], 1e-6).unwrap();
    let k = rms_norm(&[1.0, 3.0], &[1.0, 1.0], 1e-6).unwrap();
    let q = rotate_pair([q[0], q[1]], 2, 0.1);
    let k = rotate_pair([k[0], k[1]], 1, 0.1);
    let score = (q[0] * k[0] + q[1] * k[1]) / 2.0_f64.sqrt();
    assert!(score.is_finite());
    println!("QK-Norm + RoPE score={score:.4}");
}
