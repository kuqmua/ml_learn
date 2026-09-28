// Урок 35.1. Токенные и позиционные эмбеддинги.
// Вход decoder складывает представление токена и его позиции.

fn main() {
    // Строки таблиц — обучаемые параметры; здесь числа фиксированы для проверки.
    let token_embedding = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    let position_embedding = [[0.0, 0.0], [0.1, 0.0], [0.2, 0.0]];
    let token_ids = [0, 1, 0];
    let states: Vec<[f64; 2]> = token_ids
        .iter()
        .enumerate()
        .map(|(position, &token_id)| {
            [
                token_embedding[token_id][0] + position_embedding[position][0],
                token_embedding[token_id][1] + position_embedding[position][1],
            ]
        })
        .collect();
    assert_ne!(states[0], states[2]);
    println!("входные состояния: {states:?}");
}
