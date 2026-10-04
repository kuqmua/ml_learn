// Урок 36.7. Выход выбранных ветвей: выбор двух обработчиков и объединение результатов.
// Зачем здесь эта тема: Смесь экспертов даёт много специализированных ветвей без запуска каждой для
//   каждого токена.
// Почему код устроен так: Выбираем две ветви по оценкам и объединяем их ответы с весами
//   маршрутизации.
// Представь: Из четырёх экспертов считаем только два выбранных, затем смешиваем их ответы.
// Два выбранных эксперта обрабатывают токен; пример относится к MoE-вариантам Qwen3.

use l186_35_calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum::calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum;

fn main() {
    let input: [f64; 2] = [0.8, 0.2];
    let scores: [f64; 4] = [input[0], input[1], -input[0], -input[1]];
    let mut order: [usize; 4] = [0, 1, 2, 3];
    order.sort_by(|&candidate1, &candidate2| scores[candidate2].total_cmp(&scores[candidate1]));
    let selected: [usize; 2] = [order[0], order[1]];
    let expert_gain: [f64; 4] = [1.0, 2.0, -1.0, 0.5];
    let _: f64 = selected
        .iter()
        .zip(
            std::convert::TryInto::<[f64; 2]>::try_into(
                calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
                    &[scores[selected[0]], scores[selected[1]]],
                ),
            )
            .expect("ожидалось ровно две выбранные ветви"),
        )
        .map(|(&expert, weight)| weight * expert_gain[expert] * input[0])
        .sum::<f64>();
    assert_eq!(selected, [0, 1]);

    // Выполняем вычисления из примера.
    let _ = &scores;
}

// Чему учит этот урок:
// Учимся выбирать две вычислительные ветви по оценкам и смешивать их ответы с нормализованными
// весами.
// Так вход обрабатывают только выбранные ветви, а не все возможные варианты.
