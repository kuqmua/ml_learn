// Урок 38.3. Упорядоченная статистика категорий.
// Метка текущей строки не попадает в её закодированный признак.

use part_194_lesson_38_ordered_category_statistics::ordered_target_mean;
fn main() {
    let categories = ["A", "B", "A", "A", "B"];
    let targets = [1.0, 0.0, 0.0, 1.0, 1.0];
    let values = ordered_target_mean(&categories, &targets, 0.5, 2.0).unwrap();
    assert_eq!(values[0], 0.5);
    assert_eq!(values[1], 0.5);
    visualize(&values);
    for (index, value) in values.iter().enumerate() {
        println!(
            "строка {index}, категория {}, ordered mean={value:.3}",
            categories[index]
        );
    }
}

fn visualize(values: &[f64]) {
    let points: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as f64, x))
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
