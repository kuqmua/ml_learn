// Урок 16.3. Среднее целевых меток категории без метки текущей строки.
// Почему этот урок сейчас: Среднее меток категории полезно как признак, но собственная метка строки дала бы утечку.
// Почему пример устроен так: Для текущей строки используем только ранее доступные строки и начальное сглаживание.
// Метка текущей строки не попадает в её закодированный признак.

fn main() {
    lesson_trace::enable();
    let categories: [&str; 5] = ["A", "B", "A", "A", "B"];
    lesson_trace::trace_step!(categories);
    let targets: [f64; 5] = [1.0, 0.0, 0.0, 1.0, 1.0];
    lesson_trace::trace_step!(targets);
    let values: Vec<f64> = part_088_lesson_16_ordered_category_target_mean_without_current_label::ordered_category_target_mean_using_prior_rows(
        &categories,
        &targets,
        0.5,
        2.0,
    )
    .unwrap();
    lesson_trace::trace_step!(values);
    assert_eq!(values[0], 0.5);
    assert_eq!(values[1], 0.5);
    lesson_trace::disable();
    visualize_ordered_category_target_mean_without_current_label(&values);
    for (index, value) in values.iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(value);
        println!(
            "строка {index}, категория {}, ordered mean={value:.3}",
            categories[index]
        );
    }
}

fn visualize_ordered_category_target_mean_without_current_label(values: &[f64]) {
    let points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
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
