// Урок 36.7. Маршрутизация токена через двух выбранных экспертов.
// Зачем здесь эта тема: Смесь экспертов даёт много специализированных ветвей без запуска каждой для
//   каждого токена.
// Почему код устроен так: Выбираем две ветви по оценкам и объединяем их ответы с весами
//   маршрутизации.
// Представь: Из четырёх экспертов считаем только два выбранных, затем смешиваем их ответы.
// Два выбранных эксперта обрабатывают токен; пример относится к MoE-вариантам Qwen3.

fn main() {
    lesson_trace::enable();
    let input: [f64; 2] = [0.8, 0.2];
    lesson_trace::trace_step!(input);
    let scores: [f64; 4] = [input[0], input[1], -input[0], -input[1]];
    lesson_trace::trace_step!(scores);
    let mut order: [usize; 4] = [0, 1, 2, 3];
    lesson_trace::trace_step!(order);
    order.sort_by(|&first_candidate, &second_candidate| {
        scores[second_candidate].total_cmp(&scores[first_candidate])
    });
    let selected: [usize; 2] = [order[0], order[1]];
    lesson_trace::trace_step!(selected);
    let weights: Vec<f64> =
        part_182_lesson_35_causal_self_attention_over_prefix_of_tokens::softmax_probabilities_from_raw_model_scores(&[
            scores[selected[0]],
            scores[selected[1]],
        ]);
    lesson_trace::trace_step!(weights);
    // У каждого эксперта своя простая линейная функция.
    let expert_gain: [f64; 4] = [1.0, 2.0, -1.0, 0.5];
    lesson_trace::trace_step!(expert_gain);
    let output: f64 = selected
        .iter()
        .zip(weights)
        .map(|(&expert, weight)| weight * expert_gain[expert] * input[0])
        .sum::<f64>();
    lesson_trace::trace_step!(output);
    assert_eq!(selected, [0, 1]);
    println!("выбраны эксперты {selected:?}; выход={output:.4}");
    lesson_trace::disable();
    visualize_route_token_through_two_selected_experts(&scores);
}

fn visualize_route_token_through_two_selected_experts(scores: &[f64; 4]) {
    let values: [(&str, f64); 4] = [
        ("expert 0", scores[0]),
        ("expert 1", scores[1]),
        ("expert 2", scores[2]),
        ("expert 3", scores[3]),
    ];
    let path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "router",
        "Оценки маршрутизатора",
        "score",
        &values,
    )
    .expect("график");
    println!("график: {}", path.display());
}
