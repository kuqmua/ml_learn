// Урок 35.4. Объединение контекстов от нескольких способов сравнения запросов и ключей.
// Связь с принятой терминологией: Объединение нескольких голов внимания.
// Зачем здесь эта тема: Одна голова внимания даёт один набор связей; несколько голов могут выделять
//   разные отношения.
// Почему код устроен так: Считаем головы отдельно и объединяем их выходы в исходную размерность.
// Представь: Одна голова может выделить соседнее слово, другая — более далёкую связь; затем их
//   результаты объединяются.
// Разные головы получают собственные проекции и соединяются перед выходной проекцией.

use l187_35_calculate_past_context_by_summing_current_and_past_values_with_match_weights::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

fn main() {
    let input: [[f64; 2]; 3] = [[1.0, 2.0], [3.0, 1.0], [2.0, 4.0]];
    let first_attention_head: [[f64; 2]; 3] = input.map(|input_value| [input_value[0], 0.0]);
    let second_attention_head: [[f64; 2]; 3] = input.map(|input_value| [0.0, input_value[1]]);
    let first_output: [[f64; 2]; 3] =
        calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &first_attention_head,
            &first_attention_head,
            &first_attention_head,
        )
        .unwrap()
        .try_into()
        .expect("на каждую из трёх позиций приходится один выход");
    let second_output: [[f64; 2]; 3] =
        calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &second_attention_head,
            &second_attention_head,
            &second_attention_head,
        )
        .unwrap()
        .try_into()
        .expect("на каждую из трёх позиций приходится один выход");
    let combined: [[f64; 2]; 3] =
        std::array::from_fn(|index| [first_output[index][0], second_output[index][1]]);
    assert_eq!(combined[0], input[0]);
}
