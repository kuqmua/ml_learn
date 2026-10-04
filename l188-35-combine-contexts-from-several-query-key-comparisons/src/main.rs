// Урок 35.4. Объединение контекстов от нескольких способов сравнения запросов и ключей.
// Зачем здесь эта тема: Одна голова внимания даёт один набор связей; несколько голов могут выделять
//   разные отношения.
// Почему код устроен так: Считаем головы отдельно и объединяем их выходы в исходную размерность.
// Представь: Одна голова может выделить соседнее слово, другая — более далёкую связь; затем их
//   результаты объединяются.
// Разные головы получают собственные проекции и соединяются перед выходной проекцией.

use l187_35_calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches::calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

fn main() {
    let input: [[f64; 2]; 3] = [[1.0, 2.0], [3.0, 1.0], [2.0, 4.0]];
    let attention_head1: [[f64; 2]; 3] = input.map(|input_value| [input_value[0], 0.0]);
    let attention_head2: [[f64; 2]; 3] = input.map(|input_value| [0.0, input_value[1]]);
    let output1: [[f64; 2]; 3] =
        calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &attention_head1,
            &attention_head1,
            &attention_head1,
        )
        .unwrap()
        .try_into()
        .expect("ожидался один выход на каждую из трёх позиций");
    let output2: [[f64; 2]; 3] =
        calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &attention_head2,
            &attention_head2,
            &attention_head2,
        )
        .unwrap()
        .try_into()
        .expect("ожидался один выход на каждую из трёх позиций");
    assert_eq!(
        std::array::from_fn::<[f64; 2], 3, _>(|index| [output1[index][0], output2[index][1],])[0],
        input[0]
    );
}
