// Урок 36.7. Маршрутизация Mixture-of-Experts.
// Два выбранных эксперта обрабатывают токен; пример относится к MoE-вариантам Qwen3.

fn main() {
    let input = [0.8, 0.2];
    let scores: [f64; 4] = [input[0], input[1], -input[0], -input[1]];
    let mut order = [0, 1, 2, 3];
    order.sort_by(|&first_candidate, &second_candidate| {
        scores[second_candidate].total_cmp(&scores[first_candidate])
    });
    let selected = [order[0], order[1]];
    let weights = part_182_lesson_35_causal_self_attention::softmax(&[
        scores[selected[0]],
        scores[selected[1]],
    ]);
    // У каждого эксперта своя простая линейная функция.
    let expert_gain = [1.0, 2.0, -1.0, 0.5];
    let output = selected
        .iter()
        .zip(weights)
        .map(|(&expert, weight)| weight * expert_gain[expert] * input[0])
        .sum::<f64>();
    assert_eq!(selected, [0, 1]);
    println!("выбраны эксперты {selected:?}; выход={output:.4}");
    visualize(&scores);
}

fn visualize(scores: &[f64; 4]) {
    let values = [
        ("expert 0", scores[0]),
        ("expert 1", scores[1]),
        ("expert 2", scores[2]),
        ("expert 3", scores[3]),
    ];
    let path = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "router",
        "Оценки маршрутизатора",
        "score",
        &values,
    )
    .expect("график");
    println!("график: {}", path.display());
}
