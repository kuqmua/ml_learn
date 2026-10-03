// Урок 36.7. Выход выбранных ветвей: выбор двух обработчиков и объединение результатов.
// Связь с принятой терминологией: Маршрутизация токена через двух выбранных экспертов.
// Зачем здесь эта тема: Смесь экспертов даёт много специализированных ветвей без запуска каждой для
//   каждого токена.
// Почему код устроен так: Выбираем две ветви по оценкам и объединяем их ответы с весами
//   маршрутизации.
// Представь: Из четырёх экспертов считаем только два выбранных, затем смешиваем их ответы.
// Два выбранных эксперта обрабатывают токен; пример относится к MoE-вариантам Qwen3.

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares;

fn main() {
    let input: [f64; 2] = [0.8, 0.2];
    let scores: [f64; 4] = [input[0], input[1], -input[0], -input[1]];
    let mut order: [usize; 4] = [0, 1, 2, 3];
    order.sort_by(|&first_candidate, &second_candidate| {
        scores[second_candidate].total_cmp(&scores[first_candidate])
    });
    let selected: [usize; 2] = [order[0], order[1]];
    let expert_gain: [f64; 4] = [1.0, 2.0, -1.0, 0.5];
    let _: f64 = selected
        .iter()
        .zip(
            std::convert::TryInto::<[f64; 2]>::try_into(
                calculate_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum_where_weights_sum_to_1_and_larger_scores_get_larger_shares(
                    &[scores[selected[0]], scores[selected[1]]],
                ),
            )
            .expect("ожидалось ровно две выбранные ветви"),
        )
        .map(|(&expert, weight)| weight * expert_gain[expert] * input[0])
        .sum::<f64>();
    assert_eq!(selected, [0, 1]);

    plot_scores_used_to_select_two_processing_branches(&scores);
}

fn plot_scores_used_to_select_two_processing_branches(scores: &[f64; 4]) {
    let values: [(&str, f64); 4] = [
        ("expert 0", scores[0]),
        ("expert 1", scores[1]),
        ("expert 2", scores[2]),
        ("expert 3", scores[3]),
    ];
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "router",
        "Оценки маршрутизатора",
        "score",
        &values,
    )
    .expect("не удалось сохранить график");
}
