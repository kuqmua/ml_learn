// Урок 36.3. Общее представление контекста: использование одних ключей и значений для нескольких групп запросов.
// Связь с принятой терминологией: Совместное использование голов ключей и значений несколькими головами запросов.
// Зачем здесь эта тема: Для многих голов Q хранение отдельных K и V дорого.
// Почему код устроен так: Несколько голов запросов используют общие группы K/V; сравниваем число
//   векторов и выход.
// Представь: Четыре головы Q могут обращаться к меньшему числу общих наборов K/V, экономя хранение.
// Несколько Q-голов совместно используют меньшее число K/V-голов.

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum;

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Четырём Q-головам соответствуют две K/V-головы.");
    let queries: [[f64; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [-1.0, 1.0]];
    trace_step!(queries);
    let keys: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.0, 1.0], [1.0, 0.0]]];
    trace_step!(keys);
    let values: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.2, 0.8], [0.8, 0.2]]];
    trace_step!(values);
    let mut output: Vec<[f64; 2]> = Vec::new();
    trace_step!(output);
    for (head, &query) in queries.iter().enumerate() {
        trace_step!(head);
        trace_step!(query);
        let group: usize = head / 2;
        trace_step!(group);
        trace_note!("Оценку модели до преобразования в вероятность называют logit.");
        let raw_model_scores: Vec<f64> = keys[group]
            .iter()
            .map(|key_vector| query[0] * key_vector[0] + query[1] * key_vector[1])
            .collect();
        trace_step!(raw_model_scores);
        let weights: Vec<f64> =
            calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
                &raw_model_scores,
            );
        trace_step!(weights);
        output.push([
            weights[0] * values[group][0][0] + weights[1] * values[group][1][0],
            weights[0] * values[group][0][1] + weights[1] * values[group][1][1],
        ]);
    }
    assert_eq!(output.len(), 4);
    println!("4 Q / 2 KV головы: {output:?}");
}
