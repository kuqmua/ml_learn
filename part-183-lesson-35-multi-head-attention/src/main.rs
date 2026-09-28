// Урок 35.3. Несколько голов внимания.
// Разные головы получают собственные проекции и соединяются перед выходной проекцией.

use part_182_lesson_35_causal_self_attention::causal_attention;
fn main() {
    let input = [[1.0, 2.0], [3.0, 1.0], [2.0, 4.0]];
    // Первая голова смотрит на первый признак, вторая — на второй.
    let head_a: Vec<[f64; 2]> = input
        .iter()
        .map(|input_value| [input_value[0], 0.0])
        .collect();
    let head_b: Vec<[f64; 2]> = input
        .iter()
        .map(|input_value| [0.0, input_value[1]])
        .collect();
    let out_a = causal_attention(&head_a, &head_a, &head_a).unwrap();
    let out_b = causal_attention(&head_b, &head_b, &head_b).unwrap();
    // Конкатенация двух одномерных выходов здесь сразу даёт размерность 2.
    let combined: Vec<[f64; 2]> = out_a
        .iter()
        .zip(&out_b)
        .map(|(first_value, second_value)| [first_value[0], second_value[1]])
        .collect();
    assert_eq!(combined[0], input[0]);
    println!("две головы: {combined:?}");
}
