// Урок 16.3. Кодирование категории: среднее предыдущих ответов без ответа текущей строки.
// Связь с принятой терминологией: Среднее целевых меток категории без метки текущей строки.
// Зачем здесь эта тема: Среднее меток категории полезно как признак, но собственная метка строки
//   дала бы утечку.
// Почему код устроен так: Для текущей строки используем только ранее доступные строки и начальное
//   сглаживание.
// Представь: Для строки с категорией A и меткой 1 нельзя считать среднее A с учётом этой самой
//   единицы.
// Метка текущей строки не попадает в её закодированный признак.

use l092_16_encode_category_by_averaging_earlier_targets_without_current_answer::encode_categories_as_average_previous_targets_with_prior_weight;

use lesson_trace::{disable, enable, trace_step};

fn main() {
    enable();
    let categories: [&str; 5] = ["A", "B", "A", "A", "B"];
    trace_step!(categories);
    let targets: [f64; 5] = [1.0, 0.0, 0.0, 1.0, 1.0];
    trace_step!(targets);
    let values: Vec<f64> = encode_categories_as_average_previous_targets_with_prior_weight(
        &categories,
        &targets,
        0.5,
        2.0,
    )
    .unwrap();
    trace_step!(values);
    assert_eq!(values[0], 0.5);
    assert_eq!(values[1], 0.5);
    disable();
    plot_category_target_averages_using_only_previous_rows(&values);
    for (index, value) in values.iter().enumerate() {
        trace_step!(index);
        trace_step!(value);
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
