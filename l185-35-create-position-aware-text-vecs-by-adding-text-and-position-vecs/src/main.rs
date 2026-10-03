// Урок 35.1. Векторы текста с учётом позиции: сложение векторов частей текста и позиций.
// Связь с принятой терминологией: Сложение токенных и позиционных эмбеддингов на входе decoder.
// Зачем здесь эта тема: При генерации одинаковые токены в разных местах должны различаться.
// Почему код устроен так: Складываем токенный вектор с позиционным до первого слоя decoder.
// Представь: Два одинаковых слова на местах 0 и 2 получают разные входные векторы из-за позиции.
// Вход decoder складывает представление токена и его позиции.

fn main() {
    let text_unit_dense_representation: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    let position_dense_representation: [[f64; 2]; 3] = [[0.0, 0.0], [0.1, 0.0], [0.2, 0.0]];
    let text_unit_identifiers: [usize; 3] = [0, 1, 0];
    let states: [[f64; 2]; 3] = std::array::from_fn(|position| {
        let text_unit_identifier = text_unit_identifiers[position];
        [
            text_unit_dense_representation[text_unit_identifier][0]
                + position_dense_representation[position][0],
            text_unit_dense_representation[text_unit_identifier][1]
                + position_dense_representation[position][1],
        ]
    });
    assert_ne!(states[0], states[2]);
}
