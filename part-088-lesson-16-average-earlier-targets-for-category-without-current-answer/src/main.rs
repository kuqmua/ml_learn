// Урок 16.3. Усреднение предыдущих ответов категории без использования ответа текущей строки.
// Связь с принятой терминологией: Среднее целевых меток категории без метки текущей строки.
// Зачем здесь эта тема: Среднее меток категории полезно как признак, но собственная метка строки
//   дала бы утечку.
// Почему код устроен так: Для текущей строки используем только ранее доступные строки и начальное
//   сглаживание.
// Представь: Для строки с категорией A и меткой 1 нельзя считать среднее A с учётом этой самой
//   единицы.
// Метка текущей строки не попадает в её закодированный признак.

fn main() {
    lesson_trace::enable();
    let categories: [&str; 5] = ["A", "B", "A", "A", "B"];
    lesson_trace::trace_step!(categories);
    let targets: [f64; 5] = [1.0, 0.0, 0.0, 1.0, 1.0];
    lesson_trace::trace_step!(targets);
    let values: Vec<f64> = part_088_lesson_16_average_earlier_targets_for_category_without_current_answer::average_previous_targets_per_category_with_prior_weight(
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
    plot_category_target_averages_using_only_previous_rows(&values);
    for (index, value) in values.iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(value);
        println!(
            "строка {index}, категория {}, ordered mean={value:.3}",
            categories[index]
        );
    }
}

fn plot_category_target_averages_using_only_previous_rows(values: &[f64]) {
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
