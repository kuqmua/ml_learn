// Урок 16.3. Среднее целевых меток категории без метки текущей строки.
// Метка текущей строки не попадает в её закодированный признак.

fn main() {
    let categories = ["A", "B", "A", "A", "B"];
    let targets = [1.0, 0.0, 0.0, 1.0, 1.0];
    let values = part_088_lesson_16_ordered_category_target_mean_without_current_label::ordered_category_target_mean_using_prior_rows(
        &categories,
        &targets,
        0.5,
        2.0,
    )
    .unwrap();
    assert_eq!(values[0], 0.5);
    assert_eq!(values[1], 0.5);
    visualize_ordered_category_target_mean_without_current_label(&values);
    for (index, value) in values.iter().enumerate() {
        println!(
            "строка {index}, категория {}, ordered mean={value:.3}",
            categories[index]
        );
    }
}

fn visualize_ordered_category_target_mean_without_current_label(values: &[f64]) {
    let points: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "ordered-stats",
        "Префиксная статистика",
        "строка",
        "оценка",
        &[lesson_visualization::Series {
            name: "ordered mean",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
