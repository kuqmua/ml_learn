// Урок 34.1. Двунаправленное внимание между токенами в encoder.
// В отличие от decoder, текущий токен читает контекст и слева, и справа.

fn main() {
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let output: Vec<[f64; 2]> =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &states, &[true; 3],
        )
        .unwrap();
    let changed: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [9.0, 9.0]];
    let after: Vec<[f64; 2]> =
        part_178_lesson_34_bidirectional_self_attention_in_encoder::bidirectional_self_attention_over_visible_states(
            &changed, &[true; 3],
        )
        .unwrap();
    assert_ne!(output[0], after[0]);
    println!("первый токен учитывает будущий контекст: {:?}", output[0]);
}
