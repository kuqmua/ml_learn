// Урок 045. Начинаем с простого способа, с которым потом сравним обученную модель.
// Находим самый частый правильный класс в обучающих данных и всегда предсказываем его.
// Если сложная модель работает хуже этого способа, её дополнительная сложность пока не помогает.

fn main() {
    let targets: [bool; 5] = [false, false, true, false, true];
    assert!(!targets.is_empty(), "для baseline нужна хотя бы одна метка");
    let pos_count: usize = targets.iter().filter(|&&target| target).count();
    let majority_class: bool = pos_count * 2 > targets.len();
    let _: f64 = targets
        .iter()
        .filter(|&&target| target == majority_class)
        .count() as f64
        / targets.len() as f64;

    plot_counts_of_pos_and_neg_training_targets(targets, pos_count);
}

// Строим график по результатам урока.
fn plot_counts_of_pos_and_neg_training_targets(targets: [bool; 5], pos_count: usize) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Классы для baseline",
        "объектов",
        &[
            ("отрицательные", (targets.len() - pos_count) as f64),
            ("положительные", pos_count as f64),
        ],
    )
    .expect("не удалось сохранить график");
}
