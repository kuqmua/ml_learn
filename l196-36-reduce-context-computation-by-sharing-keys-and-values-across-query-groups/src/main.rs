// Урок 36.3. Общее представление контекста: использование одних ключей и значений для нескольких групп запросов.
// Зачем здесь эта тема: Для многих голов Q хранение отдельных K и V дорого.
// Почему код устроен так: Несколько голов запросов используют общие группы K/V; сравниваем число
//   векторов и выход.
// Представь: Четыре головы Q могут обращаться к меньшему числу общих наборов K/V, экономя хранение.
// Несколько Q-голов совместно используют меньшее число K/V-голов.

use l186_35_calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum;

fn main() {
    let queries: [[f64; 2]; 4] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [-1.0, 1.0]];
    let keys: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.0, 1.0], [1.0, 0.0]]];
    let values: [[[f64; 2]; 2]; 2] = [[[1.0, 0.0], [0.0, 1.0]], [[0.2, 0.8], [0.8, 0.2]]];
    assert!(
        std::array::from_fn::<[f64; 2], 4, _>(|head| {
            let query = queries[head];
            let group: usize = head / 2;
            let weights: [f64; 2] =
            calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
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
        .all(|head_output| head_output.iter().all(|value| value.is_finite()))
    );

    let separate_key_numbers = queries.len() * keys[0].len() * keys[0][0].len();
    let shared_key_numbers = keys.len() * keys[0].len() * keys[0][0].len();
    println!(
        "Для {} запросов: отдельных чисел ключей={separate_key_numbers}, с разделением на {} группы={shared_key_numbers}",
        queries.len(),
        keys.len()
    );
    assert_eq!(separate_key_numbers, 2 * shared_key_numbers);
    for head in 0..queries.len() {
        println!(
            "Запрос {head} использует группу ключей и значений {}",
            head / 2
        );
    }
}

// Чему учит этот урок:
// Учимся использовать общие ключи и значения для нескольких запросов внутри группы.
// Это показывает устройство внимания с разделяемыми данными, позволяющее хранить меньше отдельных
// наборов ключей и значений.
