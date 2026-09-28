// Урок 35.6. Учебный прямой проход GPT.
// Собираем токены, позиции, причинное внимание и выходные логиты в один decoder-only проход.

use part_186_lesson_35_tiny_gpt_forward::forward;

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
