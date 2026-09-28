// Урок 32.2. Маска padding в encoder.
// Добавленные PAD позиции не должны влиять на реальные выходные токены.

use part_168_lesson_32_bidirectional_encoder::bidirectional_attention;
fn main() {
    let real = [[1.0, 0.0], [0.0, 1.0]];
    let base = bidirectional_attention(&real, &[true, true]).unwrap();
    let padded = [[1.0, 0.0], [0.0, 1.0], [100.0, 100.0]];
    let masked = bidirectional_attention(&padded, &[true, true, false]).unwrap();
    assert_eq!(base, masked[..2]);
    println!("выход без PAD и с PAD совпадает: {base:?}");
}
