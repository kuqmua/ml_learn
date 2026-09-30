// Урок 36.3. Общее представление контекста: использование одних ключей и значений для нескольких групп запросов.
// Связь с принятой терминологией: Совместное использование голов ключей и значений несколькими головами запросов.
// Зачем здесь эта тема: Для многих голов Q хранение отдельных K и V дорого.
// Почему код устроен так: Несколько голов запросов используют общие группы K/V; сравниваем число
//   векторов и выход.
// Представь: Четыре головы Q могут обращаться к меньшему числу общих наборов K/V, экономя хранение.
// Несколько Q-голов совместно используют меньшее число K/V-голов.

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum;

fn main() {
    let queries: [[f64; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [-1.0, 1.0]];
    let keys: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.0, 1.0], [1.0, 0.0]]];
    let values: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.2, 0.8], [0.8, 0.2]]];
    let mut output: Vec<[f64; 2]> = Vec::new();
    for (head, &query) in queries.iter().enumerate() {
        let group: usize = head / 2;
        let raw_model_scores: Vec<f64> = keys[group]
            .iter()
            .map(|key_vector| query[0] * key_vector[0] + query[1] * key_vector[1])
            .collect();
        let weights: Vec<f64> =
            calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
                &raw_model_scores,
            );
        output.push([
            weights[0] * values[group][0][0] + weights[1] * values[group][1][0],
            weights[0] * values[group][0][1] + weights[1] * values[group][1][1],
        ]);
    }
    assert_eq!(output.len(), 4);
}
