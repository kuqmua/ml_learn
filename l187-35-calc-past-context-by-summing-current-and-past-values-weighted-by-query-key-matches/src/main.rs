// Урок 35.3. Контекст без будущих данных: сложение текущих и прошлых значений с весами совпадений запроса и ключей.
// Зачем здесь эта тема: Decoder предсказывает продолжение по уже известному префиксу.
// Почему код устроен так: Ограничиваем внимание текущей и прошлыми позициями и проверяем
//   независимость от будущего.
// Представь: При прогнозе слова после «добрый» decoder не должен использовать ещё неизвестное
//   продолжение.
// Для позиции i softmax вычисляется только по позициям 0..=i.

use l187_35_calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches::calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

fn main() {
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let context: [[f64; 2]; 3] =
        calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &states, &states, &states,
        )
        .unwrap()
        .try_into()
        .expect("ожидался один контекст на каждую из трёх позиций");
    assert_eq!(context[0], states[0]);

    for (index, state) in context.iter().enumerate() {
        println!("Позиция {index}: доступны 0..={index}, контекст={state:?}");
    }

    // Выполняем вычисления из примера.
    let _ = &states;
}

// Чему учит этот урок:
// Учимся строить контекст каждой позиции только из неё самой и предшествующих позиций.
// Проверяем первую позицию: без доступного прошлого её контекст совпадает с её собственным
// значением.
