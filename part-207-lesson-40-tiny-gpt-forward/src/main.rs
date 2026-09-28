// Урок 40.6. Учебный прямой проход GPT.
// Собираем токены, позиции, причинное внимание и выходные логиты в один decoder-only проход.

use part_203_lesson_40_causal_self_attention::causal_attention;

fn forward(ids: &[usize]) -> Vec<[f64; 3]> {
    let embedding = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    let states: Vec<[f64; 2]> = ids
        .iter()
        .enumerate()
        .map(|(position, &id)| [embedding[id][0] + position as f64 * 0.1, embedding[id][1]])
        .collect();
    let context = causal_attention(&states, &states, &states).unwrap();
    states
        .iter()
        .zip(&context)
        .map(|(&state, &attended)| {
            let hidden = [state[0] + attended[0], state[1] + attended[1]];
            // Выходная линейная голова для трёх токенов.
            [hidden[0], hidden[1], (hidden[0] + hidden[1]) * 0.5]
        })
        .collect()
}
fn main() {
    let prefix = [0, 1];
    let logits = forward(&prefix);
    let last = logits.last().unwrap();
    let next = last
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .unwrap()
        .0;
    // Изменение будущего токена не меняет предыдущие позиции.
    assert_eq!(forward(&[0])[0], logits[0]);
    println!("префикс: {prefix:?}; logits: {last:?}; следующий ID: {next}");
    // Весов из GPT здесь нет: это минимальный прямой проход с фиксированными параметрами.
}
