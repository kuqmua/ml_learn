// Урок 36.3. Общее представление контекста: использование одних ключей и значений для нескольких групп запросов.
// Связь с принятой терминологией: Совместное использование голов ключей и значений несколькими головами запросов.
// Зачем здесь эта тема: Для многих голов Q хранение отдельных K и V дорого.
// Почему код устроен так: Несколько голов запросов используют общие группы K/V; сравниваем число
//   векторов и выход.
// Представь: Четыре головы Q могут обращаться к меньшему числу общих наборов K/V, экономя хранение.
// Несколько Q-голов совместно используют меньшее число K/V-голов.

use l186_35_calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares;

fn main() {
    let queries: [[f64; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [-1.0, 1.0]];
    let keys: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.0, 1.0], [1.0, 0.0]]];
    let values: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.2, 0.8], [0.8, 0.2]]];
    assert!(std::array::from_fn::<[f64; 2], 4, _>(|head| {
        let query = queries[head];
        let group: usize = head / 2;
        let weights: [f64; 2] =
            calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
                &std::array::from_fn::<f64, 2, _>(|index| {
                    let key_vec = keys[group][index];
                    query[0] * key_vec[0] + query[1] * key_vec[1]
                }),
            )
            .try_into()
            .expect("ожидалось по одному весу на каждый из двух ключей");
        [
            weights[0] * values[group][0][0] + weights[1] * values[group][1][0],
            weights[0] * values[group][0][1] + weights[1] * values[group][1][1],
        ]
    })
    .iter()
    .all(|head_output| head_output.iter().all(|value| value.is_finite())));
}
