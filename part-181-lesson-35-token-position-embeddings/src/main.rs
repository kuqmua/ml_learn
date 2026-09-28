// Урок 35.1. Токенные и позиционные эмбеддинги.
// Вход decoder складывает представление токена и его позиции.

fn main() {
    // Строки таблиц — обучаемые параметры; здесь числа фиксированы для проверки.
    // Плотное числовое представление объекта называют embedding.
    let text_unit_dense_representation = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    let position_dense_representation = [[0.0, 0.0], [0.1, 0.0], [0.2, 0.0]];
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifiers = [0, 1, 0];
    let states: Vec<[f64; 2]> = text_unit_identifiers
        .iter()
        .enumerate()
        .map(|(position, &text_unit_identifier)| {
            [
                text_unit_dense_representation[text_unit_identifier][0]
                    + position_dense_representation[position][0],
                text_unit_dense_representation[text_unit_identifier][1]
                    + position_dense_representation[position][1],
            ]
        })
        .collect();
    assert_ne!(states[0], states[2]);
    println!("входные состояния: {states:?}");
}
