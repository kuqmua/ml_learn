// Контекстные векторы текста: сложение вектора токена, позиции и прошлого контекста.

use l191_35_calc_text_context_vecs_by_adding_position_and_weighted_past_context::calc_text_context_vecs_by_adding_position_and_weighted_past_context;

fn main() {
    let identifiers = [0, 1];
    let states = calc_text_context_vecs_by_adding_position_and_weighted_past_context(&identifiers);

    assert_eq!(states[0], [2.0, 0.0]);

    assert_eq!(
        states.as_slice(),
        &calc_text_context_vecs_by_adding_position_and_weighted_past_context(&[0, 1, 2])[..2]
    );
}

// Чему учит этот урок:
// Учимся получать контекстные векторы из идентификаторов с учётом позиций и прошлого.
// Проверяем, что добавление будущих токенов не меняет уже вычисленные состояния префикса.
