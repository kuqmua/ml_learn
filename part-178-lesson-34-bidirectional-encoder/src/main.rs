// Урок 34.1. Двунаправленное внимание encoder.
// В отличие от decoder, текущий токен читает контекст и слева, и справа.

fn main() {
    let states = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let output =
        part_178_lesson_34_bidirectional_encoder::bidirectional_attention(&states, &[true; 3])
            .unwrap();
    let changed = [[1.0, 0.0], [0.0, 1.0], [9.0, 9.0]];
    let after =
        part_178_lesson_34_bidirectional_encoder::bidirectional_attention(&changed, &[true; 3])
            .unwrap();
    assert_ne!(output[0], after[0]);
    println!("первый токен учитывает будущий контекст: {:?}", output[0]);
}
