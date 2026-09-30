// Урок 35.1. Векторы текста с учётом позиции: сложение векторов частей текста и позиций.
// Связь с принятой терминологией: Сложение токенных и позиционных эмбеддингов на входе decoder.
// Зачем здесь эта тема: При генерации одинаковые токены в разных местах должны различаться.
// Почему код устроен так: Складываем токенный вектор с позиционным до первого слоя decoder.
// Представь: Два одинаковых слова на местах 0 и 2 получают разные входные векторы из-за позиции.
// Вход decoder складывает представление токена и его позиции.

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Строки таблиц — обучаемые параметры; здесь числа фиксированы для проверки.");
    trace_note!("Плотное числовое представление объекта называют embedding.");
    let text_unit_dense_representation: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]];
    trace_step!(text_unit_dense_representation);
    let position_dense_representation: [[f64; 2]; 3] = [[0.0, 0.0], [0.1, 0.0], [0.2, 0.0]];
    trace_step!(position_dense_representation);
    trace_note!("Единицу текста, которую модель обрабатывает как одно целое, называют token.");
    let text_unit_identifiers: [usize; 3] = [0, 1, 0];
    trace_step!(text_unit_identifiers);
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
    trace_step!(states);
    assert_ne!(states[0], states[2]);
    println!("входные состояния: {states:?}");
}
